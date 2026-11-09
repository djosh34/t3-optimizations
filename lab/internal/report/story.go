package report

import (
	"bufio"
	"cmp"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
)

// Story runs put the untouched v0.1.7 web client and the t3opt web app side
// by side on one seed's network story, on one stock t3 server (issue #58).

// Targets of the plan, graded on every story run.
const (
	// PromptTarget is case 70's: the prompt confirmed within 10 s.
	PromptTarget = 10 * time.Second
	// LagTarget is case 50's: the newest text never more than 5 s behind
	// the server's release of it.
	LagTarget = 5 * time.Second
	// HangAfter makes a freeze a hang.
	HangAfter = 30 * time.Second
)

// Release is one line of t3-watch.mjs: an assistant message's text as the
// t3 server released it to its clients.
type Release struct {
	T       int64  `json:"t"`
	Thread  string `json:"thread"`
	Item    string `json:"item"`
	Ordinal int64  `json:"ordinal"`
	Text    string `json:"text"`
}

// ReadReleases reads t3-watch.mjs's output.
func ReadReleases(path string) ([]Release, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	var out []Release
	lines := bufio.NewScanner(f)
	lines.Buffer(make([]byte, 1<<20), 16<<20)
	for lines.Scan() {
		var r Release
		if err := json.Unmarshal(lines.Bytes(), &r); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		out = append(out, r)
	}
	return out, lines.Err()
}

// replyPieces finds the pieces of text that appear in the reply, in order,
// as [position, length]; pieces under 8 characters are skipped. It is
// driver.mjs's function of the same name, so the server's text and the
// phone's are matched alike.
func replyPieces(reply, text string) [][2]int {
	var pieces [][2]int
	rest, from := text, 0
	for len(rest) >= 8 {
		lo, hi := 0, len(rest)
		for lo < hi {
			mid := (lo + hi + 1) / 2
			if strings.Contains(reply[from:], rest[:mid]) {
				lo = mid
			} else {
				hi = mid - 1
			}
		}
		if lo >= 8 {
			at := from + strings.Index(reply[from:], rest[:lo])
			pieces = append(pieces, [2]int{at, lo})
			from = at + lo
			rest = rest[lo:]
		} else {
			rest = rest[1:]
		}
	}
	return pieces
}

// releasedSamples is how much of the reply, as a normalized prefix, the
// server had released to its clients over time: the assistant messages of
// the thread that first appeared after the prompt was sent.
func releasedSamples(rel []Release, thread string, sent time.Time, reply string) []sample {
	seen := make([]bool, len(reply))
	firstSeen := map[string]bool{}
	text := map[string]Release{}
	var out []sample
	for _, r := range rel {
		if r.Thread != thread {
			continue
		}
		if _, ok := firstSeen[r.Item]; !ok {
			firstSeen[r.Item] = at(r.T).After(sent)
		}
		if !firstSeen[r.Item] {
			continue
		}
		text[r.Item] = r
		items := make([]Release, 0, len(text))
		for _, t := range text {
			items = append(items, t)
		}
		slices.SortFunc(items, func(a, b Release) int { return cmp.Compare(a.Ordinal, b.Ordinal) })
		for _, item := range items {
			for _, p := range replyPieces(reply, agent.Normalize(item.Text)) {
				for i := p[0]; i < p[0]+p[1]; i++ {
					seen[i] = true
				}
			}
		}
		n := 0
		for n < len(seen) && seen[n] {
			n++
		}
		if len(out) == 0 || out[len(out)-1].n != n {
			out = append(out, sample{at(r.T), n})
		}
	}
	return out
}

// maxLag is the largest newest-text lag up to end, exactly: the lag grows
// until the phone shows more, so it peaks just before each new text on the
// phone, or at the end.
func maxLag(end time.Time, shown, released []sample) float64 {
	worst, have := 0.0, 0
	check := func(t time.Time) {
		if countAt(released, t) > have {
			if rt, ok := producedTime(released, have+1); ok {
				worst = max(worst, secs(t.Sub(rt)))
			}
		}
	}
	for _, s := range shown {
		if s.t.After(end) {
			break
		}
		check(s.t)
		have = max(have, s.n)
	}
	check(end)
	return worst
}

// Freeze is a stretch in which the phone showed nothing new while the
// server had released text it did not show.
type Freeze struct {
	// AtS is seconds after the prompt was sent.
	AtS   float64 `json:"atS"`
	ForS  float64 `json:"forS"`
	Ended bool    `json:"ended"`
}

// freezes finds every freeze between sent and end, on a 100 ms grid.
func freezes(sent, end time.Time, shown, released []sample) []Freeze {
	var out []Freeze
	var start time.Time
	lastShown := -1
	closeAt := func(t time.Time, ended bool) {
		if !start.IsZero() {
			out = append(out, Freeze{AtS: secs(start.Sub(sent)), ForS: secs(t.Sub(start)), Ended: ended})
			start = time.Time{}
		}
	}
	for t := sent; !t.After(end); t = t.Add(100 * time.Millisecond) {
		v, r := countAt(shown, t), countAt(released, t)
		if v != lastShown || r <= v {
			closeAt(t, true)
		}
		lastShown = v
		if r > v && start.IsZero() {
			start = t
		}
	}
	closeAt(end, false)
	return out
}

// StoryClient is one client's result on a story.
type StoryClient struct {
	Client string `json:"client"`
	Status string `json:"status"`
	// Seconds after the prompt was sent; nil if it never happened.
	PromptConfirmationS *float64 `json:"promptConfirmationS"`
	TimeToFirstTextS    *float64 `json:"timeToFirstTextS"`
	CompleteS           *float64 `json:"completeS"`
	// Newest-text lag, graded from the server's release; the lag from the
	// agent is for information. Nil when there was no text to lag behind.
	MaxReleaseLagS *float64 `json:"maxReleaseLagS"`
	MaxAgentLagS   *float64 `json:"maxAgentLagS"`
	LongestFreezeS float64  `json:"longestFreezeS"`
	// Hangs are freezes of HangAfter or more.
	Hangs []Freeze `json:"hangs"`
	// Reloads are page loads after the switch.
	Reloads int `json:"reloads"`
	// WebSocketsOpened counts the page's WebSockets after the switch: the
	// v0.1.7 client's reconnects, the web app's fallback.
	WebSocketsOpened int `json:"webSocketsOpened"`
	// Transports is the web app's transport at each connection generation.
	Transports []string `json:"transports,omitempty"`
}

// PromptMet says whether case 70's target was met.
func (c StoryClient) PromptMet() bool {
	return c.PromptConfirmationS != nil && *c.PromptConfirmationS <= PromptTarget.Seconds()
}

// LagMet says whether case 50's target was met: the whole reply shown, never
// more than 5 s behind the server.
func (c StoryClient) LagMet() bool {
	return c.CompleteS != nil && c.MaxReleaseLagS != nil && *c.MaxReleaseLagS <= LagTarget.Seconds()
}

// OnUDP says whether the web app stayed on the data channel: no WebSocket
// at all.
func (c StoryClient) OnUDP() bool {
	return c.WebSocketsOpened == 0 && !slices.Contains(c.Transports, "websocket")
}

// Load is the host's load during the run.
type Load struct {
	CPUs          int     `json:"cpus"`
	MeanLoad1     float64 `json:"meanLoad1"`
	MaxLoad1      float64 `json:"maxLoad1"`
	MinMemAvailMB int64   `json:"minMemAvailMB"`
}

// StoryResult is one seed's story run.
type StoryResult struct {
	Seed     int64         `json:"seed"`
	Survival bool          `json:"survival"`
	Commit   string        `json:"commit"`
	Load     Load          `json:"load"`
	Clients  []StoryClient `json:"clients"`
	// Phases names the story's segments, "name@seconds".
	Phases []string `json:"phases"`
}

// BuildStoryClient measures one client of a story run. releases are the
// server's releases of this client's thread.
func BuildStoryClient(in Input, rel []Release, thread string) StoryClient {
	r := Build(in)
	c := StoryClient{
		Client: in.Client, Status: r.Status,
		PromptConfirmationS: r.PromptConfirmationS, TimeToFirstTextS: r.TimeToFirstTextS,
		CompleteS: r.CompleteS, MaxAgentLagS: r.MaxLagS,
	}
	var sent time.Time
	var shown []sample
	for _, ev := range in.Phone {
		t := at(ev.T)
		switch ev.Ev {
		case "prompt-sent":
			sent = t
		case "text":
			shown = append(shown, sample{t, ev.N})
		case "page-load":
			c.Reloads++
		case "ws-open":
			if t.After(in.Switched) {
				c.WebSocketsOpened++
			}
		case "transport":
			c.Transports = append(c.Transports, ev.Name)
		}
	}
	if sent.IsZero() {
		return c
	}
	end := in.End
	if r.CompleteS != nil {
		end = sent.Add(time.Duration(*r.CompleteS * float64(time.Second)))
	}
	released := releasedSamples(rel, thread, sent, in.Reply)
	if len(released) > 0 {
		c.MaxReleaseLagS = ptr(maxLag(end, shown, released))
	}
	for _, f := range freezes(sent, end, shown, released) {
		c.LongestFreezeS = max(c.LongestFreezeS, f.ForS)
		if f.ForS >= HangAfter.Seconds() {
			c.Hangs = append(c.Hangs, f)
		}
	}
	return c
}

func yes(b bool) string {
	if b {
		return "met"
	}
	return "not met"
}

// Markdown renders one seed's run.
func (s StoryResult) Markdown() string {
	var b strings.Builder
	kind := "2G"
	if s.Survival {
		kind = "survival"
	}
	fmt.Fprintf(&b, "# Story seed %d (%s)\n\n", s.Seed, kind)
	fmt.Fprintf(&b, "Commit %s. Host: %d CPUs, load %.1f mean, %.1f max, %d MB memory available at least.\n\n",
		s.Commit, s.Load.CPUs, s.Load.MeanLoad1, s.Load.MaxLoad1, s.Load.MinMemAvailMB)
	fmt.Fprintf(&b, "Story: %s.\n\n", strings.Join(s.Phases, ", "))
	b.WriteString("| | " + strings.Join(clientNames(s.Clients), " | ") + " |\n|---|")
	b.WriteString(strings.Repeat("---|", len(s.Clients)) + "\n")
	row := func(name string, f func(StoryClient) string) {
		var cells []string
		for _, c := range s.Clients {
			cells = append(cells, f(c))
		}
		fmt.Fprintf(&b, "| %s | %s |\n", name, strings.Join(cells, " | "))
	}
	row("Status", func(c StoryClient) string { return c.Status })
	row("Prompt confirmed", func(c StoryClient) string { return fmtSecs(c.PromptConfirmationS) })
	row("Prompt within 10 s (case 70)", func(c StoryClient) string { return yes(c.PromptMet()) })
	row("First reply text", func(c StoryClient) string { return fmtSecs(c.TimeToFirstTextS) })
	row("Whole reply shown", func(c StoryClient) string { return fmtSecs(c.CompleteS) })
	row("Newest-text lag from the server's release, max", func(c StoryClient) string { return fmtLag(c.MaxReleaseLagS) })
	row("Lag within 5 s (case 50)", func(c StoryClient) string { return yes(c.LagMet()) })
	row("Newest-text lag from the agent, max", func(c StoryClient) string { return fmtLag(c.MaxAgentLagS) })
	row("Longest freeze", func(c StoryClient) string { return fmt.Sprintf("%.1fs", c.LongestFreezeS) })
	row("Hangs (freezes of 30 s or more)", func(c StoryClient) string { return fmtHangs(c.Hangs) })
	row("Reloads", func(c StoryClient) string { return fmt.Sprint(c.Reloads) })
	row("WebSockets opened after the switch", func(c StoryClient) string { return fmt.Sprint(c.WebSocketsOpened) })
	row("Transports", func(c StoryClient) string { return cmp.Or(strings.Join(c.Transports, ", "), "-") })
	return b.String()
}

func clientNames(cs []StoryClient) []string {
	var out []string
	for _, c := range cs {
		out = append(out, c.Client)
	}
	return out
}

func fmtHangs(hs []Freeze) string {
	if len(hs) == 0 {
		return "none"
	}
	var parts []string
	for _, h := range hs {
		end := ""
		if !h.Ended {
			end = ", to the end"
		}
		parts = append(parts, fmt.Sprintf("%.0f s at %.0f s%s", h.ForS, h.AtS, end))
	}
	return strings.Join(parts, "; ")
}

// Write writes story.json and story.md to dir.
func (s StoryResult) Write(dir string, write func(path string, data []byte) error) error {
	data, err := json.MarshalIndent(s, "", "  ")
	if err != nil {
		return err
	}
	if err := write(filepath.Join(dir, "story.json"), append(data, '\n')); err != nil {
		return err
	}
	return write(filepath.Join(dir, "story.md"), []byte(s.Markdown()))
}

// StorySummary prints Markdown over every story.json under dir: both
// clients over all seeds, and the worst seeds.
func StorySummary(dir string) error {
	paths, err := filepath.Glob(filepath.Join(dir, "*", "story.json"))
	if err != nil {
		return err
	}
	var runs []StoryResult
	for _, p := range paths {
		data, err := os.ReadFile(p)
		if err != nil {
			return err
		}
		var s StoryResult
		if err := json.Unmarshal(data, &s); err != nil {
			return fmt.Errorf("%s: %w", p, err)
		}
		runs = append(runs, s)
	}
	if len(runs) == 0 {
		return fmt.Errorf("no story.json under %s", dir)
	}
	slices.SortFunc(runs, func(a, b StoryResult) int { return cmp.Compare(a.Seed, b.Seed) })
	fmt.Print(storySummary(runs))
	return nil
}

func storySummary(runs []StoryResult) string {
	var b strings.Builder
	survival, commits := 0, []string{}
	for _, r := range runs {
		if r.Survival {
			survival++
		}
		if !slices.Contains(commits, r.Commit) {
			commits = append(commits, r.Commit)
		}
	}
	fmt.Fprintf(&b, "%d seeds (%d survival stories), lab commit %s.\n\n", len(runs), survival, strings.Join(commits, ", "))
	names := clientNames(runs[0].Clients)
	b.WriteString("| Over all seeds | " + strings.Join(names, " | ") + " |\n|---|" + strings.Repeat("---|", len(names)) + "\n")
	per := func(i int) []StoryClient {
		var out []StoryClient
		for _, r := range runs {
			if i < len(r.Clients) {
				out = append(out, r.Clients[i])
			}
		}
		return out
	}
	row := func(name string, f func([]StoryClient) string) {
		var cells []string
		for i := range names {
			cells = append(cells, f(per(i)))
		}
		fmt.Fprintf(&b, "| %s | %s |\n", name, strings.Join(cells, " | "))
	}
	share := func(f func(StoryClient) bool) func([]StoryClient) string {
		return func(cs []StoryClient) string {
			n := 0
			for _, c := range cs {
				if f(c) {
					n++
				}
			}
			return fmt.Sprintf("%d/%d", n, len(cs))
		}
	}
	spread := func(f func(StoryClient) float64) func([]StoryClient) string {
		return func(cs []StoryClient) string {
			var v []float64
			for _, c := range cs {
				v = append(v, f(c))
			}
			maxV, median, p95 := quantiles(v)
			return fmt.Sprintf("%.1f / %.1f / %.1f s", median, p95, maxV)
		}
	}
	total := func(f func(StoryClient) int) func([]StoryClient) string {
		return func(cs []StoryClient) string {
			n := 0
			for _, c := range cs {
				n += f(c)
			}
			return fmt.Sprint(n)
		}
	}
	row("Reply completed by the deadline", share(func(c StoryClient) bool { return c.CompleteS != nil }))
	row("Prompt confirmed within 10 s (case 70)", share(StoryClient.PromptMet))
	row("Newest text within 5 s of the server's release (case 50)", share(StoryClient.LagMet))
	row("Max lag from the server's release, median / p95 / worst", spread(func(c StoryClient) float64 {
		if c.MaxReleaseLagS == nil {
			return 0
		}
		return *c.MaxReleaseLagS
	}))
	row("Longest freeze, median / p95 / worst", spread(func(c StoryClient) float64 { return c.LongestFreezeS }))
	row("Hangs (freezes of 30 s or more)", total(func(c StoryClient) int { return len(c.Hangs) }))
	row("Reloads", total(func(c StoryClient) int { return c.Reloads }))
	row("WebSockets opened after the switch", total(func(c StoryClient) int { return c.WebSocketsOpened }))
	row("Stayed on the data channel (UDP), no WebSocket", share(StoryClient.OnUDP))

	b.WriteString("\nWorst seeds, by the t3opt web app's longest freeze:\n\n")
	b.WriteString("| Seed | Story | " + strings.Join(names, " status / freeze | ") + " status / freeze |\n|---|---|" + strings.Repeat("---|", len(names)) + "\n")
	worst := slices.Clone(runs)
	slices.SortStableFunc(worst, func(a, b StoryResult) int {
		return cmp.Compare(appFreeze(b), appFreeze(a))
	})
	for _, r := range worst[:min(10, len(worst))] {
		kind := "2G"
		if r.Survival {
			kind = "survival"
		}
		var cells []string
		for _, c := range r.Clients {
			cells = append(cells, fmt.Sprintf("%s / %.0f s", c.Status, c.LongestFreezeS))
		}
		fmt.Fprintf(&b, "| [%d](s%d/story.md) | %s | %s |\n", r.Seed, r.Seed, kind, strings.Join(cells, " | "))
	}
	return b.String()
}

func appFreeze(r StoryResult) float64 {
	for _, c := range r.Clients {
		if c.Client == "webapp" {
			return c.LongestFreezeS
		}
	}
	return 0
}
