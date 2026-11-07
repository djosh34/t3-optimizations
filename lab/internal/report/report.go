// Package report turns a run's raw events into its report, in JSON and
// Markdown.
package report

import (
	"cmp"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"sort"
	"strings"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
	"github.com/djosh34/t3-optimizations/lab/internal/profile"
)

// Status values.
const (
	StatusDone        = "done"
	StatusNotDone     = "not done by deadline"
	StatusNeverLoaded = "app never loaded by deadline"
	StatusNotSent     = "prompt never sent by deadline"
)

// PathNote tells the reader what the lab's TCP is not.
const PathNote = "Kernel TCP over a veth pair in the lab. In production the phone's TCP ends in " +
	"tailscaled's userspace stack (issue #7), so production numbers differ."

// Input is everything a run recorded.
type Input struct {
	Profile, Grade, About string
	Client, Kind          string
	Seed                  int64
	Commit                string
	Deadline              time.Duration
	// Switched is when the lab applied the profile; End is when the run ended.
	Switched, End time.Time
	// Reply is the normalized measured reply.
	Reply string
	Agent []agent.LogEvent
	Phone []phone.Event
	// WireDown and WireUp are bytes on the link after the switch.
	WireDown, WireUp uint64
}

// LagPoint is the newest-text lag at one moment.
type LagPoint struct {
	// AtS is seconds after the prompt was sent.
	AtS float64 `json:"atS"`
	// LagS is how old the oldest text the agent produced but the phone has
	// not shown is, in seconds; 0 when the phone shows everything produced.
	LagS float64 `json:"lagS"`
	// Shown and Produced are normalized character counts.
	Shown    int `json:"shown"`
	Produced int `json:"produced"`
}

// Report is the result of one run. Durations are in seconds; a nil field
// means the event never happened.
type Report struct {
	Profile  string `json:"profile"`
	About    string `json:"about"`
	Grade    string `json:"grade"`
	Client   string `json:"client"`
	Kind     string `json:"kind"`
	Seed     int64  `json:"seed"`
	Commit   string `json:"commit"`
	Path     string `json:"path"`
	Status   string `json:"status"`
	Deadline string `json:"deadline"`
	// RunS is the time from the profile switch to the end of the run.
	RunS float64 `json:"runS"`

	AppLoadS            *float64 `json:"appLoadS,omitempty"`
	PromptConfirmationS *float64 `json:"promptConfirmationS"`
	TimeToFirstTextS    *float64 `json:"timeToFirstTextS"`
	CompleteS           *float64 `json:"completeS"`
	// AgentGotPromptS is when the prompt reached the agent.
	AgentGotPromptS *float64 `json:"agentGotPromptS"`
	// The lag fields are nil when the agent produced no text: there was
	// nothing to fall behind on, which is not the same as no lag.
	MaxLagS             *float64 `json:"maxLagS"`
	MedianLagS          *float64 `json:"medianLagS"`
	P95LagS             *float64 `json:"p95LagS"`
	LongestSilenceS     float64  `json:"longestSilenceS"`
	ShownChars          int      `json:"shownChars"`
	ProducedChars       int      `json:"producedChars"`
	ReplyChars          int      `json:"replyChars"`
	ApprovalShownAfterS *float64 `json:"approvalShownAfterS"`

	WireDownBytes               uint64 `json:"wireDownBytes"`
	WireUpBytes                 uint64 `json:"wireUpBytes"`
	AppReceivedBytes            int64  `json:"appReceivedBytes"`
	AppSentBytes                int64  `json:"appSentBytes"`
	ReplyBytesReceived          int64  `json:"replyBytesReceived"`
	DuplicateReplyBytes         int64  `json:"duplicateReplyBytes"`
	WebSocketsOpenedAfterSwitch int    `json:"webSocketsOpenedAfterSwitch"`

	Lag   []LagPoint `json:"lag"`
	Notes []string   `json:"notes,omitempty"`
}

func secs(d time.Duration) float64 { return float64(d.Milliseconds()) / 1000 }

func ptr(v float64) *float64 { return &v }

func at(ms int64) time.Time { return time.UnixMilli(ms) }

// Build computes the report.
func Build(in Input) Report {
	r := Report{
		Profile: in.Profile, About: in.About, Grade: in.Grade,
		Client: in.Client, Kind: in.Kind, Seed: in.Seed, Commit: in.Commit,
		Path:       PathNote,
		Deadline:   in.Deadline.String(),
		RunS:       secs(in.End.Sub(in.Switched)),
		ReplyChars: len(in.Reply),
	}
	var sent, approvalAsked, approvalShown time.Time
	var shown []sample
	loaded := in.Kind != "cold"
	for _, ev := range in.Phone {
		t := at(ev.T)
		switch ev.Ev {
		case "app-loaded":
			loaded = true
			r.AppLoadS = ptr(secs(t.Sub(in.Switched)))
		case "prompt-sent":
			sent = t
		case "prompt-confirmed":
			if !sent.IsZero() {
				r.PromptConfirmationS = ptr(secs(t.Sub(sent)))
			}
		case "text":
			shown = append(shown, sample{t, ev.N})
		case "complete":
			if !sent.IsZero() {
				r.CompleteS = ptr(secs(t.Sub(sent)))
			}
		case "approval-shown":
			if approvalShown.IsZero() {
				approvalShown = t
			}
		case "ws-open":
			if t.After(in.Switched) {
				r.WebSocketsOpenedAfterSwitch++
			}
		case "stats":
			r.AppReceivedBytes = ev.RxBytes
			r.AppSentBytes = ev.TxBytes
			r.ReplyBytesReceived = ev.ReplyBytes
			r.DuplicateReplyBytes = ev.DuplicateReplyBytes
		case "note":
			r.Notes = append(r.Notes, ev.Text)
		}
	}
	r.WireDownBytes, r.WireUpBytes = in.WireDown, in.WireUp

	produced, gotPrompt := producedSamples(in.Agent, &approvalAsked)
	if !approvalAsked.IsZero() && !approvalShown.IsZero() {
		r.ApprovalShownAfterS = ptr(secs(approvalShown.Sub(approvalAsked)))
	}
	if len(produced) > 0 {
		r.ProducedChars = produced[len(produced)-1].n
	}
	if len(shown) > 0 {
		r.ShownChars = shown[len(shown)-1].n
	}
	switch {
	case r.CompleteS != nil:
		r.Status = StatusDone
	case !loaded:
		r.Status = StatusNeverLoaded
	case sent.IsZero():
		r.Status = StatusNotSent
	default:
		r.Status = StatusNotDone
	}
	if sent.IsZero() {
		return r
	}
	for _, s := range shown {
		if s.n > 0 {
			r.TimeToFirstTextS = ptr(secs(s.t.Sub(sent)))
			break
		}
	}
	if !gotPrompt.IsZero() {
		r.AgentGotPromptS = ptr(secs(gotPrompt.Sub(sent)))
	}
	r.Lag = lagSeries(sent, in.End, shown, produced)
	if len(produced) > 0 {
		var lags []float64
		for _, p := range r.Lag {
			lags = append(lags, p.LagS)
		}
		maxV, median, p95 := quantiles(lags)
		r.MaxLagS, r.MedianLagS, r.P95LagS = ptr(maxV), ptr(median), ptr(p95)
	}
	end := in.End
	if r.CompleteS != nil {
		end = sent.Add(time.Duration(*r.CompleteS * float64(time.Second)))
	}
	r.LongestSilenceS = longestSilence(sent, end, shown)
	return r
}

type sample struct {
	t time.Time
	n int
}

// producedSamples returns the agent's text events of the measured turn and
// when the measured prompt reached the agent, and sets approvalAsked to when
// the agent asked for the approval.
func producedSamples(log []agent.LogEvent, approvalAsked *time.Time) (out []sample, gotPrompt time.Time) {
	measured := false
	for _, ev := range log {
		switch ev.Ev {
		case "prompt":
			measured = strings.Contains(ev.Prompt, agent.MeasuredPrompt)
			if measured && gotPrompt.IsZero() {
				gotPrompt = at(ev.T)
			}
		case "text":
			if measured {
				out = append(out, sample{at(ev.T), ev.N})
			}
		case "approval-asked":
			if measured {
				*approvalAsked = at(ev.T)
			}
		}
	}
	return out, gotPrompt
}

// countAt is the count of the last sample at or before t.
func countAt(samples []sample, t time.Time) int {
	i := sort.Search(len(samples), func(i int) bool { return samples[i].t.After(t) })
	if i == 0 {
		return 0
	}
	return samples[i-1].n
}

// producedTime is when the agent produced character k (1-based).
func producedTime(produced []sample, k int) (time.Time, bool) {
	i := sort.Search(len(produced), func(i int) bool { return produced[i].n >= k })
	if i == len(produced) {
		return time.Time{}, false
	}
	return produced[i].t, true
}

func lagAt(t time.Time, shown, produced []sample) LagPoint {
	v, p := countAt(shown, t), countAt(produced, t)
	point := LagPoint{Shown: v, Produced: p}
	if v < p {
		if pt, ok := producedTime(produced, v+1); ok {
			point.LagS = secs(t.Sub(pt))
		}
	}
	return point
}

// lagSeries samples the lag every second from the prompt to the end.
func lagSeries(sent, end time.Time, shown, produced []sample) []LagPoint {
	var out []LagPoint
	for t := sent; !t.After(end); t = t.Add(time.Second) {
		p := lagAt(t, shown, produced)
		p.AtS = secs(t.Sub(sent))
		out = append(out, p)
	}
	return out
}

// longestSilence is the longest time after the prompt was sent in which the
// phone showed no new reply text, up to the end of the reply or the run.
func longestSilence(sent, end time.Time, shown []sample) float64 {
	longest, last, lastN := 0.0, sent, 0
	for _, s := range shown {
		if s.n <= lastN {
			continue
		}
		longest = max(longest, secs(s.t.Sub(last)))
		last, lastN = s.t, s.n
	}
	return max(longest, secs(end.Sub(last)))
}

func quantiles(values []float64) (maxV, median, p95 float64) {
	if len(values) == 0 {
		return 0, 0, 0
	}
	sorted := slices.Clone(values)
	slices.Sort(sorted)
	pick := func(q float64) float64 { return sorted[int(q*float64(len(sorted)-1))] }
	return sorted[len(sorted)-1], pick(0.5), pick(0.95)
}

// Headline is a one-line summary.
func (r Report) Headline() string {
	return fmt.Sprintf("%s %s %s seed %d: %s, first text %s, prompt confirmed %s, max lag %s, longest silence %.1fs",
		r.Profile, r.Client, r.Kind, r.Seed, r.Status,
		fmtSecs(r.TimeToFirstTextS), fmtSecs(r.PromptConfirmationS), fmtLag(r.MaxLagS), r.LongestSilenceS)
}

// fmtLag shows a lag, or that the agent produced nothing to lag behind.
func fmtLag(v *float64) string {
	if v == nil {
		return "no text produced"
	}
	return fmt.Sprintf("%.1fs", *v)
}

func fmtSecs(v *float64) string {
	if v == nil {
		return "never"
	}
	return fmt.Sprintf("%.1fs", *v)
}

// Name is the file name stem of a report.
func (r Report) Name() string {
	return fmt.Sprintf("%s-%s-%s", r.Profile, r.Client, r.Kind)
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

// Markdown renders the report for people.
func (r Report) Markdown() string {
	var b strings.Builder
	fmt.Fprintf(&b, "# %s, %s, %s start, seed %d\n\n", r.Profile, r.Client, r.Kind, r.Seed)
	fmt.Fprintf(&b, "Profile %s (%s): %s. Commit %s.\n\n", r.Profile, r.Grade, r.About, r.Commit)
	fmt.Fprintf(&b, "%s\n\n", r.Path)
	fmt.Fprintf(&b, "**Status: %s** after %.0f s (deadline %s).\n\n", r.Status, r.RunS, r.Deadline)
	b.WriteString("| Measure | Value |\n|---|---|\n")
	row := func(name, value string) { fmt.Fprintf(&b, "| %s | %s |\n", name, value) }
	if r.Kind == "cold" {
		row("App loaded after the switch", fmtSecs(r.AppLoadS))
	}
	row("Prompt confirmation", fmtSecs(r.PromptConfirmationS))
	row("Prompt reached the agent", fmtSecs(r.AgentGotPromptS))
	row("Time to first text", fmtSecs(r.TimeToFirstTextS))
	row("Whole reply shown", fmtSecs(r.CompleteS))
	row("Reply shown / produced / total (chars)", fmt.Sprintf("%d / %d / %d", r.ShownChars, r.ProducedChars, r.ReplyChars))
	row("Newest-text lag: max / median / p95", fmt.Sprintf("%s / %s / %s", fmtLag(r.MaxLagS), fmtLag(r.MedianLagS), fmtLag(r.P95LagS)))
	row("Longest silence", fmt.Sprintf("%.1f s", r.LongestSilenceS))
	row("Approval shown after it was asked", fmtSecs(r.ApprovalShownAfterS))
	row("Bytes on the wire down / up", fmt.Sprintf("%d / %d", r.WireDownBytes, r.WireUpBytes))
	row("Bytes the app received / sent", fmt.Sprintf("%d / %d", r.AppReceivedBytes, r.AppSentBytes))
	row("Reply text received / of it duplicates", fmt.Sprintf("%d / %d", r.ReplyBytesReceived, r.DuplicateReplyBytes))
	row("WebSockets opened after the switch", fmt.Sprint(r.WebSocketsOpenedAfterSwitch))
	b.WriteString("\n## Newest-text lag over time\n\nEvery 10 s after the prompt was sent.\n\n")
	b.WriteString("| After (s) | Lag (s) | Shown | Produced |\n|---|---|---|---|\n")
	for i, p := range r.Lag {
		if i%10 == 0 || i == len(r.Lag)-1 {
			fmt.Fprintf(&b, "| %.0f | %.1f | %d | %d |\n", p.AtS, p.LagS, p.Shown, p.Produced)
		}
	}
	if len(r.Notes) > 0 {
		b.WriteString("\n## Driver notes\n\n")
		for _, n := range r.Notes {
			fmt.Fprintf(&b, "- %s\n", n)
		}
	}
	return b.String()
}

// Summary prints a Markdown table of every report.json under dir.
func Summary(dir string) error {
	paths, err := filepath.Glob(filepath.Join(dir, "*", "report.json"))
	if err != nil {
		return err
	}
	var reports []Report
	for _, path := range paths {
		data, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		var r Report
		if err := json.Unmarshal(data, &r); err != nil {
			return fmt.Errorf("%s: %w", path, err)
		}
		reports = append(reports, r)
	}
	order := func(name string) int {
		return slices.IndexFunc(profile.All, func(p profile.Profile) bool { return p.Name == name })
	}
	// Profile order, warm before cold, then client.
	slices.SortStableFunc(reports, func(a, b Report) int {
		return cmp.Or(order(a.Profile)-order(b.Profile), strings.Compare(b.Kind, a.Kind), strings.Compare(a.Client, b.Client))
	})
	fmt.Println("| Profile | Grade | Client | Start | Status | Prompt confirmed | Prompt reached agent | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Reply received/duplicates (B) |")
	fmt.Println("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
	for _, r := range reports {
		fmt.Printf("| [%s](%s/report.md) | %s | %s | %s | %s | %s | %s | %s | %s | %.1fs | %d/%d/%d | %d/%d | %d/%d |\n",
			r.Profile, r.Name(), r.Grade, r.Client, r.Kind, r.Status,
			fmtSecs(r.PromptConfirmationS), fmtSecs(r.AgentGotPromptS), fmtSecs(r.TimeToFirstTextS),
			fmtLag(r.MaxLagS), r.LongestSilenceS, r.ShownChars, r.ProducedChars, r.ReplyChars,
			r.WireDownBytes, r.WireUpBytes, r.ReplyBytesReceived, r.DuplicateReplyBytes)
	}
	return nil
}
