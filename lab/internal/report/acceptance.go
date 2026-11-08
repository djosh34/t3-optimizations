package report

import (
	"bufio"
	"cmp"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strconv"
	"strings"

	"github.com/djosh34/t3-optimizations/lab/internal/phone"
	"github.com/djosh34/t3-optimizations/lab/internal/profile"
)

// The plan's targets for the web app (issue #46).
const (
	case70PromptS = 10.0
	case50LagS    = 5.0
)

// accepted is one web app run with what its sidecar log says about the
// backfill.
type accepted struct {
	Report
	dir string
	// Backfill bytes the sidecar still had to send when the prompt was
	// confirmed and when the first reply text showed; -1 when unknown.
	backfillAtConfirm, backfillAtText int64
}

// Acceptance prints the acceptance report of issue #46: each target of the
// web app with its result, then every web app run next to the baseline
// clients' run on the same profile, start and seed.
func Acceptance(dir, baselineDir string) error {
	runs, err := loadAccepted(dir)
	if err != nil {
		return err
	}
	baseline, err := loadReports(baselineDir)
	if err != nil {
		return err
	}
	var commits []string
	for _, r := range runs {
		if !slices.Contains(commits, r.Commit) {
			commits = append(commits, r.Commit)
		}
	}
	slices.Sort(commits)
	fmt.Printf("Commit of these runs: %s.\n\n", strings.Join(commits, ", "))
	fmt.Print(targets(runs))
	fmt.Println()
	fmt.Println("## Every run against the baseline")
	fmt.Println()
	fmt.Println("Each web app run, and below it the WebSocket baseline on the same profile, start and seed (`websocket`, `websocket-nodeflate`). Prompt is from Enter to the confirmation; text is from Enter to the first reply text on screen; lag is the newest-text lag; silence is the longest time with no new reply text.")
	fmt.Println()
	fmt.Println("| Run | Grade | Client | Status | Prompt | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Received/duplicates (B) |")
	fmt.Println("|---|---|---|---|---|---|---|---|---|---|---|")
	for _, r := range runs {
		fmt.Println(row(fmt.Sprintf("[%s](%s/report.md)", r.Name(), filepath.Base(r.dir)), r.Report))
		for _, client := range []string{"websocket", "websocket-nodeflate"} {
			i := slices.IndexFunc(baseline, func(b Report) bool {
				return b.Profile == r.Profile && b.Kind == r.Kind && b.Seed == r.Seed && b.Client == client
			})
			if i >= 0 {
				b := baseline[i]
				fmt.Println(row(fmt.Sprintf("[baseline](../baseline/%s/report.md)", b.Name()), b))
			}
		}
	}
	return nil
}

func row(name string, r Report) string {
	return fmt.Sprintf("| %s | %s | %s | %s | %s | %s | %s | %s | %d/%d/%d | %d/%d | %d/%d |",
		name, r.Grade, r.Client, r.Status, fmtSecs(r.PromptConfirmationS), fmtSecs(r.TimeToFirstTextS),
		fmtLag(r.MaxLagS), fmtSilence(r.LongestSilenceS), r.ShownChars, r.ProducedChars, r.ReplyChars,
		r.WireDownBytes, r.WireUpBytes, r.ReplyBytesReceived, r.DuplicateReplyBytes)
}

// targets checks each target of issue #46 on the web app's runs.
func targets(runs []accepted) string {
	var b strings.Builder
	line := func(format string, args ...any) { fmt.Fprintf(&b, format+"\n", args...) }
	verdict := func(ok bool) string {
		if ok {
			return "**met**"
		}
		return "**not met**"
	}
	of := func(keep func(accepted) bool) []accepted {
		var out []accepted
		for _, r := range runs {
			if keep(r) {
				out = append(out, r)
			}
		}
		return out
	}
	list := func(rs []accepted, value func(accepted) string) string {
		if len(rs) == 0 {
			return "No runs"
		}
		var parts []string
		for _, r := range rs {
			parts = append(parts, fmt.Sprintf("%s %s", r.Name(), value(r)))
		}
		return strings.Join(parts, ", ")
	}

	line("## Targets")
	line("")

	// The two cases start warm (lab/README.md); their cold starts get a
	// verdict of their own.
	for _, kind := range []string{"warm", "cold"} {
		c70 := of(func(r accepted) bool { return r.Profile == "case70" && r.Kind == kind })
		ok := len(c70) > 0
		for _, r := range c70 {
			ok = ok && r.PromptConfirmationS != nil && *r.PromptConfirmationS <= case70PromptS
		}
		line("- Case 70 from a %s start, a typed prompt confirmed within %.0f s: %s. %s.", kind, case70PromptS, verdict(ok),
			list(c70, func(r accepted) string { return fmtSecs(r.PromptConfirmationS) }))
	}
	for _, kind := range []string{"warm", "cold"} {
		c50 := of(func(r accepted) bool { return r.Profile == "case50" && r.Kind == kind })
		ok := len(c50) > 0
		for _, r := range c50 {
			ok = ok && r.MaxLagS != nil && *r.MaxLagS <= case50LagS
		}
		line("- Case 50 from a %s start, the newest text never more than %.0f s behind: %s. Max lag: %s.", kind, case50LagS, verdict(ok),
			list(c50, func(r accepted) string { return fmtLag(r.MaxLagS) }))
	}

	ae := of(func(r accepted) bool { return slices.Contains([]string{"A", "B", "C", "D", "E"}, r.Profile) })
	ok := len(ae) > 0
	for _, r := range ae {
		ok = ok && r.PromptConfirmationS != nil && r.Status == StatusDone
	}
	line("- Profiles A to E, text keeps moving and the prompt is confirmed before the deadline (the whole reply shown): %s. %s.",
		verdict(ok), list(ae, func(r accepted) string {
			return fmt.Sprintf("%s, prompt %s", r.Status, fmtSecs(r.PromptConfirmationS))
		}))

	dup := int64(0)
	for _, r := range runs {
		dup += r.DuplicateReplyBytes
	}
	line("- No content delivered twice to the application layer: %s. Bytes sync-core handed to the web app twice, over all %d runs: %d.",
		verdict(dup == 0), len(runs), dup)

	out := of(func(r accepted) bool {
		p := profileOf(r.Profile)
		return r.Profile != "F" && (p.Outages.Down > 0 || p.AddrChange > 0)
	})
	ok = len(out) > 0
	for _, r := range out {
		ok = ok && r.Status == StatusDone
	}
	line("- After every outage, sync resumes with no user action (runs with outages or an address change, the whole reply shown): %s. %s.",
		verdict(ok), list(out, func(r accepted) string { return r.Status }))

	var shown, untold []accepted
	for _, r := range runs {
		switch {
		case r.backfillAtConfirm < 0 && r.backfillAtText < 0:
		case r.backfillAtConfirm > 0 || r.backfillAtText > 0:
			shown = append(shown, r)
		default:
			untold = append(untold, r)
		}
	}
	line("- The newest text and prompt acks overtake backfill in every run: %s. In %d runs the phone had not stored everything yet when the confirmation or the first text reached it (the sidecar's unsynced bytes, from its stats lines every 10 s), so lower classes were still waiting; in %d it cannot be told, because everything was stored by the sidecar's last stats line before: %s. Per run below.",
		verdict(len(shown) > 0 && len(untold) == 0), len(shown), len(untold),
		names(untold))

	var again []accepted
	for _, r := range runs {
		if n := enterAgain(r.Report); n > 0 {
			again = append(again, r)
		}
	}
	line("- Open defect: the web app's composer ignored Enter, and the driver pressed it again, in %d of %d runs: %s. A user who does not press again never gets the prompt sent. The prompt's times count from the first Enter.",
		len(again), len(runs), list(again, func(r accepted) string { return fmt.Sprintf("%d more", enterAgain(r.Report)) }))
	line("")
	line("Profile F and `derp` are reported with no target. Bytes the phone had not stored at the confirmation / at the first text (the sidecar's last stats line before each):")
	line("")
	for _, r := range runs {
		line("- %s: %s / %s", r.Name(), fmtBytes(r.backfillAtConfirm), fmtBytes(r.backfillAtText))
	}
	return b.String()
}

func names(rs []accepted) string {
	if len(rs) == 0 {
		return "none"
	}
	var out []string
	for _, r := range rs {
		out = append(out, r.Name())
	}
	return strings.Join(out, ", ")
}

// enterAgain is how often the driver pressed Enter again in a run.
func enterAgain(r Report) int {
	n := 0
	for _, note := range r.Notes {
		if strings.Contains(note, "Enter again") {
			n++
		}
	}
	return n
}

func fmtBytes(v int64) string {
	if v < 0 {
		return "n/a"
	}
	return strconv.FormatInt(v, 10)
}

func profileOf(name string) profile.Profile {
	i := slices.IndexFunc(profile.All, func(p profile.Profile) bool { return p.Name == name })
	if i < 0 {
		return profile.Profile{}
	}
	return profile.All[i]
}

func loadReports(dir string) ([]Report, error) {
	paths, err := filepath.Glob(filepath.Join(dir, "*", "report.json"))
	if err != nil {
		return nil, err
	}
	var reports []Report
	for _, path := range paths {
		data, err := os.ReadFile(path)
		if err != nil {
			return nil, err
		}
		var r Report
		if err := json.Unmarshal(data, &r); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		reports = append(reports, r)
	}
	return reports, nil
}

func loadAccepted(dir string) ([]accepted, error) {
	paths, err := filepath.Glob(filepath.Join(dir, "*", "report.json"))
	if err != nil {
		return nil, err
	}
	var runs []accepted
	for _, path := range paths {
		data, err := os.ReadFile(path)
		if err != nil {
			return nil, err
		}
		a := accepted{dir: filepath.Dir(path)}
		if err := json.Unmarshal(data, &a.Report); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		a.backfillAtConfirm, a.backfillAtText, err = backfill(a.dir)
		if err != nil {
			return nil, err
		}
		runs = append(runs, a)
	}
	order := func(name string) int {
		return slices.IndexFunc(profile.All, func(p profile.Profile) bool { return p.Name == name })
	}
	slices.SortStableFunc(runs, func(a, b accepted) int {
		return cmp.Or(order(a.Profile)-order(b.Profile), strings.Compare(b.Kind, a.Kind), cmp.Compare(a.Seed, b.Seed))
	})
	return runs, nil
}

// statsLine matches the sidecar's stats lines: their time and the bytes the
// phone has not stored yet.
var statsLine = regexp.MustCompile(`^sidecar: (\d+) device \S+ stats: .*unsynced (\d+) B`)

// backfill reads the run's sidecar log and phone events and returns the
// bytes the sidecar still had to send at the last stats line before the
// prompt's confirmation and before the first reply text; -1 where the event
// or a stats line before it is missing.
func backfill(dir string) (atConfirm, atText int64, err error) {
	atConfirm, atText = -1, -1
	events, err := readEvents(filepath.Join(dir, "phone.jsonl"))
	if err != nil {
		return 0, 0, err
	}
	var confirmed, text int64
	sent := false
	for _, ev := range events {
		switch {
		case ev.Ev == "prompt-sent":
			sent = true
		case ev.Ev == "prompt-confirmed" && confirmed == 0:
			confirmed = ev.T
		case ev.Ev == "text" && sent && ev.N > 0 && text == 0:
			text = ev.T
		}
	}
	f, err := os.Open(filepath.Join(dir, "sidecar.log"))
	if os.IsNotExist(err) {
		return atConfirm, atText, nil
	}
	if err != nil {
		return 0, 0, err
	}
	defer f.Close()
	s := bufio.NewScanner(f)
	for s.Scan() {
		m := statsLine.FindStringSubmatch(s.Text())
		if m == nil {
			continue
		}
		t, err1 := strconv.ParseInt(m[1], 10, 64)
		n, err2 := strconv.ParseInt(m[2], 10, 64)
		if err1 != nil || err2 != nil {
			continue
		}
		if confirmed > 0 && t <= confirmed {
			atConfirm = n
		}
		if text > 0 && t <= text {
			atText = n
		}
	}
	return atConfirm, atText, s.Err()
}

func readEvents(path string) ([]phone.Event, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var out []phone.Event
	for line := range strings.SplitSeq(strings.TrimSpace(string(data)), "\n") {
		var ev phone.Event
		if err := json.Unmarshal([]byte(line), &ev); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		out = append(out, ev)
	}
	return out, nil
}
