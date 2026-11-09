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

// StoryClients are the clients of a story run, in report order: the
// untouched v0.1.7 web client and the t3opt web app.
var StoryClients = []string{"websocket", "webapp"}

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
	// NoPrompt marks the whole run of a client whose prompt never went out.
	NoPrompt bool `json:"noPrompt,omitempty"`
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
	// RunS is the time from the switch to the end of the client's run.
	RunS float64 `json:"runS"`
	// Seconds after the prompt was sent; nil if it never happened.
	PromptConfirmationS *float64 `json:"promptConfirmationS"`
	TimeToFirstTextS    *float64 `json:"timeToFirstTextS"`
	CompleteS           *float64 `json:"completeS"`
	// Newest-text lag, graded from the server's release; the lag from the
	// agent is for information. Nil when there was no text to lag behind.
	MaxReleaseLagS *float64 `json:"maxReleaseLagS"`
	MaxAgentLagS   *float64 `json:"maxAgentLagS"`
	// LongestFreezeS is the longest freeze; for a client that did not show
	// the whole reply, at least its silence to the end of the run.
	LongestFreezeS float64 `json:"longestFreezeS"`
	// Hangs are freezes of HangAfter or more.
	Hangs []Freeze `json:"hangs"`
	// Reloads are page loads after the switch.
	Reloads int `json:"reloads"`
	// WebSocketsOpened counts the page's WebSockets after the switch: the
	// v0.1.7 client's reconnects; for the web app, its probes for blocked
	// UDP, which carry nothing unless it moves to them.
	WebSocketsOpened int `json:"webSocketsOpened"`
	// Transports is the web app's transport each time it or the connection
	// changed: "websocket" means it moved to TCP.
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

// OnUDP says whether the web app stayed on the data channel, never moving
// to the WebSocket.
func (c StoryClient) OnUDP() bool {
	return c.Client == "webapp" && len(c.Transports) > 0 && !slices.Contains(c.Transports, "websocket")
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
	Seed     int64  `json:"seed"`
	Survival bool   `json:"survival"`
	Commit   string `json:"commit"`
	// DeadlineS is the run's deadline after the switch.
	DeadlineS float64       `json:"deadlineS"`
	Load      Load          `json:"load"`
	Clients   []StoryClient `json:"clients"`
	// Phases names the story's segments, "name@seconds".
	Phases []string `json:"phases"`
	// Dir is the run's directory name in a reports dir.
	Dir string `json:"-"`
}

// BuildStoryClient measures one client of a story run. releases are the
// server's releases of this client's thread.
func BuildStoryClient(in Input, rel []Release, thread string) StoryClient {
	r := Build(in)
	c := StoryClient{
		Client: in.Client, Status: r.Status, RunS: r.RunS,
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
		// The prompt never went out: frozen for the whole run.
		c.LongestFreezeS = r.RunS
		if r.RunS >= HangAfter.Seconds() {
			c.Hangs = []Freeze{{ForS: r.RunS, NoPrompt: true}}
		}
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
	found := freezes(sent, end, shown, released)
	// Not done: the silence since the phone last showed new text counts,
	// also when the server had released nothing new (the prompt never got
	// through, say).
	if r.CompleteS == nil {
		last := sent
		for _, s := range shown {
			last = s.t
		}
		silence := Freeze{AtS: secs(last.Sub(sent)), ForS: secs(end.Sub(last))}
		if n := len(found); n > 0 && !found[n-1].Ended {
			found[n-1] = silence
		} else {
			found = append(found, silence)
		}
	}
	for _, f := range found {
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
	row("WebSockets opened after the switch (v0.1.7: reconnects; web app: probes)", func(c StoryClient) string { return fmt.Sprint(c.WebSocketsOpened) })
	row("Transports", func(c StoryClient) string { return cmp.Or(strings.Join(c.Transports, ", "), "-") })
	row("Stayed on the data channel (UDP)", func(c StoryClient) string {
		if c.Client != "webapp" {
			return "-"
		}
		return yes(c.OnUDP())
	})
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
		if h.NoPrompt {
			parts = append(parts, fmt.Sprintf("%.0f s, the whole run: the prompt never went out", h.ForS))
			continue
		}
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
// clients over all seeds, every hang and reload, and the worst seeds. With
// expected > 0, seeds 1 to expected without a story.json are listed as
// failed runs and count as failures.
func StorySummary(dir string, expected int64) error {
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
		s.Dir = filepath.Base(filepath.Dir(p))
		runs = append(runs, s)
	}
	if len(runs) == 0 && expected == 0 {
		return fmt.Errorf("no story.json under %s", dir)
	}
	fmt.Print(storySummary(runs, expected))
	return nil
}

// runFailed is the status of a client whose run left no story.json.
const runFailed = "run failed"

// failedRun stands for a seed whose run left no story.json: both clients
// failed, with nothing measured.
func failedRun(seed int64) StoryResult {
	r := StoryResult{Seed: seed}
	for _, c := range StoryClients {
		r.Clients = append(r.Clients, StoryClient{Client: c, Status: runFailed})
	}
	return r
}

func storySummary(runs []StoryResult, expected int64) string {
	var b strings.Builder
	var missing []string
	for seed := int64(1); seed <= expected; seed++ {
		if !slices.ContainsFunc(runs, func(r StoryResult) bool { return r.Seed == seed }) {
			missing = append(missing, fmt.Sprint(seed))
			runs = append(runs, failedRun(seed))
		}
	}
	slices.SortFunc(runs, func(a, b StoryResult) int { return cmp.Compare(a.Seed, b.Seed) })
	if len(missing) > 0 {
		fmt.Fprintf(&b, "**Runs that failed and left no report, counted as failures for both clients:** seeds %s.\n\n", strings.Join(missing, ", "))
	}
	survival, commits := 0, []string{}
	for _, r := range runs {
		if r.Survival {
			survival++
		}
		if r.Commit != "" && !slices.Contains(commits, r.Commit) {
			commits = append(commits, r.Commit)
		}
	}
	fmt.Fprintf(&b, "%d seeds (%d survival stories), lab commit %s.\n\n", len(runs), survival, cmp.Or(strings.Join(commits, ", "), "none"))
	b.WriteString("| Over all seeds | " + strings.Join(StoryClients, " | ") + " |\n|---|" + strings.Repeat("---|", len(StoryClients)) + "\n")
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
		for i := range StoryClients {
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
	row("Max lag from the server's release, median / p95 / worst", releaseLagSpread)
	row("Longest freeze, median / p95 / worst", freezeSpread)
	row("Hangs (freezes of 30 s or more)", total(func(c StoryClient) int { return len(c.Hangs) }))
	row("Reloads", total(func(c StoryClient) int { return c.Reloads }))
	row("WebSockets opened after the switch (v0.1.7: reconnects; web app: probes)", total(func(c StoryClient) int { return c.WebSocketsOpened }))
	row("Stayed on the data channel (UDP), never on the WebSocket", func(cs []StoryClient) string {
		if len(cs) == 0 || cs[0].Client != "webapp" {
			return "-"
		}
		return share(StoryClient.OnUDP)(cs)
	})

	b.WriteString("\nEvery hang (a freeze of 30 s or more, seconds after the prompt) and reload:\n\n")
	b.WriteString("| Seed | Client | Hangs | Reloads |\n|---|---|---|---|\n")
	for _, r := range runs {
		for _, c := range r.Clients {
			if len(c.Hangs) > 0 || c.Reloads > 0 {
				fmt.Fprintf(&b, "| %d | %s | %s | %d |\n", r.Seed, c.Client, fmtHangs(c.Hangs), c.Reloads)
			}
		}
	}
	b.WriteString("\nWorst seeds, by the t3opt web app's longest freeze (failed runs are listed above):\n\n")
	b.WriteString("| Seed | Story | " + strings.Join(StoryClients, " status / freeze | ") + " status / freeze |\n|---|---|" + strings.Repeat("---|", len(StoryClients)) + "\n")
	worst := slices.DeleteFunc(slices.Clone(runs), func(r StoryResult) bool {
		return slices.ContainsFunc(r.Clients, func(c StoryClient) bool { return c.Status == runFailed })
	})
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
		seed := fmt.Sprint(r.Seed)
		if r.Dir != "" {
			seed = fmt.Sprintf("[%d](%s/story.md)", r.Seed, r.Dir)
		}
		fmt.Fprintf(&b, "| %s | %s | %s |\n", seed, kind, strings.Join(cells, " | "))
	}
	return b.String()
}

// freezeSpread spreads the longest freezes of the runs that left a report;
// a failed run measured nothing and is counted instead.
func freezeSpread(cs []StoryClient) string {
	var v []float64
	for _, c := range cs {
		if c.Status != runFailed {
			v = append(v, c.LongestFreezeS)
		}
	}
	out := "none measured"
	if len(v) > 0 {
		maxV, median, p95 := quantiles(v)
		out = fmt.Sprintf("%.1f / %.1f / %.1f s", median, p95, maxV)
	}
	if n := len(cs) - len(v); n > 0 {
		out += fmt.Sprintf(" (failed runs: %d)", n)
	}
	return out
}

// releaseLagSpread spreads the measured lags. A client the server released
// no text for has none, and a failed run measured nothing; both are
// counted, not taken as 0 (they fail the other rows).
func releaseLagSpread(cs []StoryClient) string {
	var v []float64
	noText, failed := 0, 0
	for _, c := range cs {
		switch {
		case c.Status == runFailed:
			failed++
		case c.MaxReleaseLagS == nil:
			noText++
		default:
			v = append(v, *c.MaxReleaseLagS)
		}
	}
	out := "none measured"
	if len(v) > 0 {
		maxV, median, p95 := quantiles(v)
		out = fmt.Sprintf("%.1f / %.1f / %.1f s", median, p95, maxV)
	}
	if noText > 0 {
		out += fmt.Sprintf(" (without any released text: %d)", noText)
	}
	if failed > 0 {
		out += fmt.Sprintf(" (failed runs: %d)", failed)
	}
	return out
}

func appFreeze(r StoryResult) float64 {
	for _, c := range r.Clients {
		if c.Client == "webapp" {
			return c.LongestFreezeS
		}
	}
	return 0
}
