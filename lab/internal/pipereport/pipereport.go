// Package pipereport measures a pipe run: how tiny messages crossed the
// shaped link in each direction, from the ticks both ends sent and received.
package pipereport

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/phone"
)

// Clients that run a pipe instead of the t3 web client.
const (
	// ClientPipe is the data channel on str0m with the patched SCTP layer.
	ClientPipe = "pipe"
	// ClientPipeUnpatched is the same with sctp-proto as published.
	ClientPipeUnpatched = "pipe-unpatched"
	// ClientUDP is plain UDP, the comparison without WebRTC.
	ClientUDP = "udp"
)

// IsPipeClient reports whether client is one of the pipe clients.
func IsPipeClient(client string) bool {
	return client == ClientPipe || client == ClientPipeUnpatched || client == ClientUDP
}

// Event is one line of a tick log: the driver's phone.jsonl or the server's
// server.jsonl.
type Event = phone.Event

// Input is everything a pipe run recorded.
type Input struct {
	Profile, Grade, About string
	Client                string
	Seed                  int64
	Commit                string
	// Switched is when the lab applied the profile; End is when it stopped
	// the run.
	Switched, End time.Time
	// Phone is the driver's log, Server the server's.
	Phone, Server []Event
	// WireDown and WireUp are bytes on the link after the switch.
	WireDown, WireUp uint64
}

// Direction is what one direction delivered after the switch.
type Direction struct {
	// Sent counts ticks sent after the switch.
	Sent int `json:"sent"`
	// Received counts those of them that arrived by the end of the run.
	Received int `json:"received"`
	// DeliveredPct is Received as a percentage of Sent.
	DeliveredPct float64 `json:"deliveredPct"`
	// LongestGapS is the longest time without a tick arriving, from the
	// switch to the end. LongestGapAtS is when it started, after the switch.
	LongestGapS   float64 `json:"longestGapS"`
	LongestGapAtS float64 `json:"longestGapAtS"`
	// GapsOver2S counts gaps longer than 2 s.
	GapsOver2S int `json:"gapsOver2S"`
	// Age is how old a tick was when it arrived: arrival minus send time.
	// Nil when nothing arrived.
	AgeMedianS *float64 `json:"ageMedianS"`
	AgeP95S    *float64 `json:"ageP95S"`
	AgeMaxS    *float64 `json:"ageMaxS"`
}

// Report is the result of one pipe run.
type Report struct {
	Profile string `json:"profile"`
	About   string `json:"about"`
	Grade   string `json:"grade"`
	Client  string `json:"client"`
	Seed    int64  `json:"seed"`
	Commit  string `json:"commit"`
	// RunS is the time from the switch to the end of the run.
	RunS float64 `json:"runS"`
	// Down is server to phone, Up phone to server.
	Down Direction `json:"down"`
	Up   Direction `json:"up"`

	WireDownBytes uint64 `json:"wireDownBytes"`
	WireUpBytes   uint64 `json:"wireUpBytes"`
	// Notes are the connection's state changes and the ends' notes, with
	// their time after the switch.
	Notes []string `json:"notes,omitempty"`
}

func secs(d time.Duration) float64 { return float64(d.Milliseconds()) / 1000 }

// Build computes the report.
func Build(in Input) Report {
	r := Report{
		Profile: in.Profile, About: in.About, Grade: in.Grade,
		Client: in.Client, Seed: in.Seed, Commit: in.Commit,
		RunS:          secs(in.End.Sub(in.Switched)),
		Down:          direction(in.Server, in.Phone, in.Switched, in.End),
		Up:            direction(in.Phone, in.Server, in.Switched, in.End),
		WireDownBytes: in.WireDown,
		WireUpBytes:   in.WireUp,
	}
	for _, side := range []struct {
		name   string
		events []Event
	}{{"phone", in.Phone}, {"server", in.Server}} {
		for _, ev := range side.events {
			if !slices.Contains([]string{"state", "ice", "note", "open", "closed"}, ev.Ev) {
				continue
			}
			text := ev.Ev
			if ev.Text != "" {
				text += " " + ev.Text
			}
			at := secs(time.UnixMilli(ev.T).Sub(in.Switched))
			r.Notes = append(r.Notes, fmt.Sprintf("%+.1fs %s: %s", at, side.name, text))
		}
	}
	slices.SortStableFunc(r.Notes, func(a, b string) int { return compareNoteTime(a, b) })
	return r
}

// compareNoteTime orders notes by the time they start with.
func compareNoteTime(a, b string) int {
	var ta, tb float64
	if _, err := fmt.Sscanf(a, "%fs", &ta); err != nil {
		return 0
	}
	if _, err := fmt.Sscanf(b, "%fs", &tb); err != nil {
		return 0
	}
	switch {
	case ta < tb:
		return -1
	case ta > tb:
		return 1
	}
	return 0
}

// direction measures the ticks the sender sent after the switch and what
// the receiver got of them.
func direction(sender, receiver []Event, switched, end time.Time) Direction {
	from, to := switched.UnixMilli(), end.UnixMilli()
	var d Direction
	sent := map[int64]bool{}
	for _, ev := range sender {
		if ev.Ev == "sent" && ev.Seq != nil && ev.T >= from && ev.T <= to {
			sent[*ev.Seq] = true
		}
	}
	d.Sent = len(sent)
	arrivals := []int64{from}
	var ages []float64
	got := map[int64]bool{}
	for _, ev := range receiver {
		if ev.Ev != "recv" || ev.Seq == nil || ev.T < from || ev.T > to {
			continue
		}
		arrivals = append(arrivals, ev.T)
		ages = append(ages, float64(ev.T-ev.Sent)/1000)
		if ev.Sent >= from && !got[*ev.Seq] {
			got[*ev.Seq] = true
			d.Received++
		}
	}
	if d.Sent > 0 {
		d.DeliveredPct = 100 * float64(d.Received) / float64(d.Sent)
	}
	arrivals = append(arrivals, to)
	slices.Sort(arrivals)
	for i := 1; i < len(arrivals); i++ {
		gap := float64(arrivals[i]-arrivals[i-1]) / 1000
		if gap > d.LongestGapS {
			d.LongestGapS = gap
			d.LongestGapAtS = float64(arrivals[i-1]-from) / 1000
		}
		if gap > 2 {
			d.GapsOver2S++
		}
	}
	if len(ages) > 0 {
		slices.Sort(ages)
		d.AgeMedianS = ptr(quantile(ages, 0.5))
		d.AgeP95S = ptr(quantile(ages, 0.95))
		d.AgeMaxS = ptr(ages[len(ages)-1])
	}
	return d
}

func ptr(v float64) *float64 { return &v }

// quantile of sorted values, nearest rank.
func quantile(sorted []float64, q float64) float64 {
	i := int(q*float64(len(sorted))+0.5) - 1
	return sorted[max(0, min(i, len(sorted)-1))]
}

// ReadLog reads a tick log, one event per line.
func ReadLog(path string) ([]Event, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var out []Event
	for i, line := range strings.Split(string(data), "\n") {
		if strings.TrimSpace(line) == "" {
			continue
		}
		var ev Event
		if err := json.Unmarshal([]byte(line), &ev); err != nil {
			return nil, fmt.Errorf("%s:%d: %w", path, i+1, err)
		}
		out = append(out, ev)
	}
	return out, nil
}

// Headline is the one-line result printed after a run.
func (r Report) Headline() string {
	return fmt.Sprintf("%s %s seed %d: down longest gap %.1fs, %.0f%% delivered; up longest gap %.1fs, %.0f%% delivered",
		r.Profile, r.Client, r.Seed, r.Down.LongestGapS, r.Down.DeliveredPct, r.Up.LongestGapS, r.Up.DeliveredPct)
}

// Write writes report.json and report.md to dir.
func (r Report) Write(dir string, write func(path string, data []byte) error) error {
	data, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		return err
	}
	if err := write(filepath.Join(dir, "report.json"), append(data, '\n')); err != nil {
		return err
	}
	return write(filepath.Join(dir, "report.md"), []byte(r.Markdown()))
}

func fmtAge(v *float64) string {
	if v == nil {
		return "none arrived"
	}
	return fmt.Sprintf("%.2fs", *v)
}

// Markdown renders the report for people.
func (r Report) Markdown() string {
	var b strings.Builder
	fmt.Fprintf(&b, "# Pipe: %s, %s, seed %d\n\n", r.Profile, r.Client, r.Seed)
	fmt.Fprintf(&b, "Profile %s (%s): %s. Commit %s.\n\n", r.Profile, r.Grade, r.About, r.Commit)
	fmt.Fprintf(&b, "A 25-byte tick every 100 ms each way for %.0f s after the switch to the profile.\n\n", r.RunS)
	b.WriteString("| Measure | Down (server to phone) | Up (phone to server) |\n|---|---|---|\n")
	row := func(name string, f func(Direction) string) {
		fmt.Fprintf(&b, "| %s | %s | %s |\n", name, f(r.Down), f(r.Up))
	}
	row("Ticks sent / received", func(d Direction) string { return fmt.Sprintf("%d / %d", d.Sent, d.Received) })
	row("Delivered", func(d Direction) string { return fmt.Sprintf("%.1f%%", d.DeliveredPct) })
	row("Longest gap", func(d Direction) string {
		return fmt.Sprintf("%.2fs (from %.1fs)", d.LongestGapS, d.LongestGapAtS)
	})
	row("Gaps over 2 s", func(d Direction) string { return fmt.Sprint(d.GapsOver2S) })
	row("Tick age: median / p95 / max", func(d Direction) string {
		return fmt.Sprintf("%s / %s / %s", fmtAge(d.AgeMedianS), fmtAge(d.AgeP95S), fmtAge(d.AgeMaxS))
	})
	fmt.Fprintf(&b, "\nBytes on the wire down / up: %d / %d.\n", r.WireDownBytes, r.WireUpBytes)
	if len(r.Notes) > 0 {
		b.WriteString("\n## Connection notes\n\nTime after the switch.\n\n")
		for _, n := range r.Notes {
			fmt.Fprintf(&b, "- %s\n", n)
		}
	}
	return b.String()
}

// Summary prints a Markdown table of every pipe report.json under dir.
func Summary(dir string) error {
	paths, err := filepath.Glob(filepath.Join(dir, "*", "report.json"))
	if err != nil {
		return err
	}
	fmt.Println("| Run | Client | Profile | Seed | Down: longest gap / delivered / age p95 | Up: longest gap / delivered / age p95 |")
	fmt.Println("|---|---|---|---|---|---|")
	for _, path := range paths {
		data, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		var r Report
		if err := json.Unmarshal(data, &r); err != nil {
			return fmt.Errorf("%s: %w", path, err)
		}
		if !IsPipeClient(r.Client) {
			return fmt.Errorf("%s is not a pipe report", path)
		}
		run := filepath.Base(filepath.Dir(path))
		cell := func(d Direction) string {
			return fmt.Sprintf("%.2fs / %.1f%% / %s", d.LongestGapS, d.DeliveredPct, fmtAge(d.AgeP95S))
		}
		fmt.Printf("| [%s](%s/report.md) | %s | %s | %d | %s | %s |\n", run, run, r.Client, r.Profile, r.Seed, cell(r.Down), cell(r.Up))
	}
	return nil
}
