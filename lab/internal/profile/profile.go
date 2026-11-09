// Package profile holds the link profiles the lab can apply and turns each
// into tc netem arguments.
package profile

import (
	"fmt"
	"slices"
	"strconv"
	"time"
)

// Link is one direction of the shaped link.
type Link struct {
	// RateBits is the link rate in bits per second.
	RateBits int64
	// Delay and Jitter are one-way; jitter has a normal distribution.
	Delay, Jitter time.Duration
	// Loss is a netem loss model, such as {"random", "70%"} or
	// {"gemodel", "15%", "5%", "100%", "0%"}. Empty means no loss.
	Loss []string
	// Limit is the queue length in packets.
	Limit int
	// Reorder, Duplicate and Corrupt are netem arguments, such as "25% 50%".
	// Empty means none. Reorder turns off the FIFO that otherwise keeps jitter
	// from reordering packets.
	Reorder, Duplicate, Corrupt []string
}

// Outages describes when the link goes silent in both directions. The first
// outage starts at First after the profile is applied and lasts Down. With a
// Period, it repeats every Period from the start of one outage to the next.
type Outages struct {
	First, Down, Period time.Duration
}

// A Phase is one stretch of a profile with phases: from At after the profile
// is applied until the next phase, the link is Down and Up.
type Phase struct {
	At       time.Duration
	Name     string
	Down, Up Link
}

// Profile is a named set of link conditions and faults.
type Profile struct {
	Name  string
	About string
	// Down is server to phone, Up is phone to server.
	Down, Up Link
	Outages  Outages
	// AddrChange, when set, moves the phone to a new address this long after
	// the profile is applied, like a new NAT mapping.
	AddrChange time.Duration
	// TCPPath sends all phone traffic through one TCP connection across the
	// shaped link, like a Tailscale DERP relay.
	TCPPath bool
	// Graded profiles carry the pass numbers of the plan. Profile F is
	// diagnostic: always run and reported, never graded.
	Graded bool
	// Soak runs one long turn for an hour.
	Soak bool
	// Phases, when set, change the link as the run goes on: a network story
	// (FromStory). Down and Up are the first phase's link.
	Phases []Phase
	// RunFor overrides the deadline when set.
	RunFor time.Duration
}

// Deadline is how long a run on this profile may take after the profile is
// applied.
func (p Profile) Deadline() time.Duration {
	if p.RunFor > 0 {
		return p.RunFor
	}
	if p.Soak {
		return 60 * time.Minute
	}
	return 15 * time.Minute
}

// Grade says how the report treats the profile.
func (p Profile) Grade() string {
	switch {
	case p.Graded:
		return "graded"
	case p.Name == "F":
		return "diagnostic"
	}
	return "fault"
}

// NetemArgs returns the arguments after "netem" for one direction.
func (l Link) NetemArgs(seed int64) []string {
	args := []string{"limit", strconv.Itoa(l.Limit)}
	if l.Delay > 0 {
		args = append(args, "delay", ms(l.Delay))
		if l.Jitter > 0 {
			args = append(args, ms(l.Jitter), "distribution", "normal")
		}
	}
	if len(l.Loss) > 0 {
		args = append(append(args, "loss"), l.Loss...)
	}
	if len(l.Reorder) > 0 {
		args = append(append(args, "reorder"), l.Reorder...)
	}
	if len(l.Duplicate) > 0 {
		args = append(append(args, "duplicate"), l.Duplicate...)
	}
	if len(l.Corrupt) > 0 {
		args = append(append(args, "corrupt"), l.Corrupt...)
	}
	return append(args, "rate", fmt.Sprintf("%dbit", l.RateBits), "seed", strconv.FormatInt(seed, 10))
}

func ms(d time.Duration) string {
	return fmt.Sprintf("%dms", d.Milliseconds())
}

// OutageAt reports whether the link is down at offset t after the profile
// was applied.
func (o Outages) OutageAt(t time.Duration) bool {
	if o.Down == 0 || t < o.First {
		return false
	}
	since := t - o.First
	if o.Period > 0 {
		since %= o.Period
	}
	return since < o.Down
}

// PhaseAt is the index of the phase at offset t after the profile was
// applied.
func (p Profile) PhaseAt(t time.Duration) int {
	i := 0
	for j, ph := range p.Phases {
		if t >= ph.At {
			i = j
		}
	}
	return i
}

// Get returns the profile with this name.
func Get(name string) (Profile, error) {
	i := slices.IndexFunc(All, func(p Profile) bool { return p.Name == name })
	if i < 0 {
		return Profile{}, fmt.Errorf("unknown profile %q; run `lab profiles` for the list", name)
	}
	return All[i], nil
}

const s = time.Second
const msec = time.Millisecond

func ge(p, r string) []string    { return []string{"gemodel", p, r, "100%", "0%"} }
func random(pct string) []string { return []string{"random", pct} }

func link(rateBits int64, delay, jitter time.Duration, loss []string, limit int) Link {
	return Link{RateBits: rateBits, Delay: delay, Jitter: jitter, Loss: loss, Limit: limit}
}

func both(l Link) (Link, Link) { return l, l }

// The fault profiles share this link unless they change it: an ordinary weak
// mobile link, so the fault itself is what the run measures.
var weak = link(200_000, 150*msec, 20*msec, random("1%"), 32)

func withFault(p Profile) Profile {
	if p.Down.RateBits == 0 {
		p.Down, p.Up = both(weak)
	}
	return p
}

func lossy(rateBits int64, loss []string, limit int) (Link, Link) {
	return both(link(rateBits, 300*msec, 50*msec, loss, limit))
}

func profile(name, about string, down, up Link) Profile {
	return Profile{Name: name, About: about, Down: down, Up: up}
}

// All lists every profile. A to F come from issue #15; the graded cases and
// the fault profiles from issue #18 and lab/README.md.
var All = func() []Profile {
	list := []Profile{
		profile("A", "weak 4G, still useful",
			link(10_000_000, 35*msec, 10*msec, random("0.15%"), 128),
			link(1_000_000, 35*msec, 10*msec, random("0.15%"), 128)),
		profile("B", "congested LTE, queue and stall stress",
			link(512_000, 500*msec, 100*msec, ge("0.5%", "10%"), 64),
			link(128_000, 500*msec, 100*msec, ge("0.5%", "10%"), 64)),
		profile("C", "historical EDGE-like",
			link(200_000, 450*msec, 50*msec, ge("0.5%", "25%"), 32),
			link(114_000, 450*msec, 50*msec, ge("0.5%", "25%"), 32)),
		profile("D", "worst observed EDGE, inspired envelope",
			link(64_000, 4000*msec, 500*msec, ge("2%", "10%"), 64),
			link(32_000, 4000*msec, 500*msec, ge("2%", "10%"), 64)),
		profile("E", "owner extreme: 1 kbit/s, 75% bursty loss",
			link(1_000, 2000*msec, 500*msec, ge("15%", "5%"), 2),
			link(1_000, 2000*msec, 500*msec, ge("15%", "5%"), 2)),
		profile("E-iid", "E with independent 75% loss instead of bursts",
			link(1_000, 2000*msec, 500*msec, random("75%"), 2),
			link(1_000, 2000*msec, 500*msec, random("75%"), 2)),
		profile("F", "64 bit/s, 75% bursty loss, down 20 s of every 30 s",
			link(64, 2000*msec, 500*msec, ge("15%", "5%"), 2),
			link(64, 2000*msec, 500*msec, ge("15%", "5%"), 2)),
	}
	list[1].Outages = Outages{First: 118 * s, Down: 2 * s, Period: 120 * s}
	list[3].Outages = Outages{First: 54 * s, Down: 6 * s, Period: 60 * s}
	list[4].Outages = Outages{First: 50 * s, Down: 10 * s, Period: 60 * s}
	list[5].Outages = list[4].Outages
	list[6].Outages = Outages{First: 10 * s, Down: 20 * s, Period: 30 * s}
	for i := range 5 {
		list[i].Graded = true
	}

	case70down, case70up := lossy(10_000, random("70%"), 20)
	case50down, case50up := lossy(100_000, random("50%"), 50)
	loss30down, loss30up := lossy(100_000, random("30%"), 50)
	loss70down, loss70up := lossy(100_000, random("70%"), 50)
	loss90down, loss90up := lossy(100_000, random("90%"), 50)
	burst90down, burst90up := lossy(100_000, ge("45%", "5%"), 50)
	rate100down, rate100up := lossy(800, nil, 4)
	reorderDown, reorderUp := lossy(100_000, random("10%"), 50)
	reorderDown.Reorder, reorderUp.Reorder = []string{"25%", "50%"}, []string{"25%", "50%"}
	dupDown, dupUp := lossy(100_000, random("5%"), 50)
	dupDown.Duplicate, dupUp.Duplicate = []string{"20%"}, []string{"20%"}
	corruptDown, corruptUp := lossy(100_000, nil, 50)
	corruptDown.Corrupt, corruptUp.Corrupt = []string{"10%"}, []string{"10%"}
	rttDown, rttUp := both(link(200_000, 1500*msec, 300*msec, random("1%"), 64))
	derpDown, derpUp := both(link(200_000, 150*msec, 20*msec, random("10%"), 32))

	list = append(list,
		Profile{Name: "case70", About: "graded: 70% loss, 10 kbit/s", Down: case70down, Up: case70up, Graded: true},
		Profile{Name: "case50", About: "graded: 50% loss, 100 kbit/s", Down: case50down, Up: case50up, Graded: true},
		profile("loss30", "30% random loss, 100 kbit/s", loss30down, loss30up),
		profile("loss70", "70% random loss, 100 kbit/s", loss70down, loss70up),
		profile("loss90", "90% random loss, 100 kbit/s", loss90down, loss90up),
		profile("burst90", "90% bursty loss (bad runs of 20 packets), 100 kbit/s", burst90down, burst90up),
		profile("rate100", "100 bytes/s, no random loss", rate100down, rate100up),
		profile("rtt3000", "3 s round trip with 300 ms jitter, 200 kbit/s", rttDown, rttUp),
		profile("reorder", "25% of packets reordered, 10% loss, 100 kbit/s", reorderDown, reorderUp),
		profile("duplicate", "20% of packets duplicated, 5% loss, 100 kbit/s", dupDown, dupUp),
		profile("corrupt", "10% of packets corrupted, 100 kbit/s", corruptDown, corruptUp),
		profile("asym", "uplink 2 kbit/s with 50% loss, downlink 200 kbit/s",
			link(200_000, 150*msec, 20*msec, random("1%"), 32),
			link(2_000, 400*msec, 100*msec, random("50%"), 8)),
		withFault(Profile{Name: "outage10", About: "one 10 s outage on a weak link", Outages: Outages{First: 10 * s, Down: 10 * s}}),
		withFault(Profile{Name: "outage60", About: "one 60 s outage on a weak link", Outages: Outages{First: 10 * s, Down: 60 * s}}),
		withFault(Profile{Name: "flap", About: "down 5 s of every 15 s on a weak link", Outages: Outages{First: 10 * s, Down: 5 * s, Period: 15 * s}}),
		withFault(Profile{Name: "addrchange", About: "the phone's address changes once, on a weak link", AddrChange: 15 * s}),
		Profile{Name: "derp", About: "all traffic in one TCP connection over a 10% loss link, like a DERP relay", Down: derpDown, Up: derpUp, TCPPath: true},
		Profile{Name: "soak", About: "one hour-long turn at 50% loss, 100 kbit/s", Down: case50down, Up: case50up, Soak: true},
	)
	return list
}()
