package report

import (
	"cmp"
	"fmt"
	"slices"
	"strings"

	"github.com/djosh34/t3-optimizations/lab/internal/profile"
)

// ServerRunName is the directory of a run in a server comparison: profile,
// server and seed.
func ServerRunName(profileName, server string, seed int64) string {
	return fmt.Sprintf("%s-%s-s%d", profileName, server, seed)
}

// ServerCompare prints the comparison of issue #47: every profile and seed
// run against the untouched v0.1.7 server and against the forked one, the
// two rows next to each other, then totals for each server.
func ServerCompare(dir string) error {
	reports, err := loadReports(dir)
	if err != nil {
		return err
	}
	order := func(name string) int {
		return slices.IndexFunc(profile.All, func(p profile.Profile) bool { return p.Name == name })
	}
	// Profile order, then seed, v0.1.7 before the fork.
	slices.SortStableFunc(reports, func(a, b Report) int {
		return cmp.Or(order(a.Profile)-order(b.Profile), cmp.Compare(a.Seed, b.Seed), strings.Compare(b.Server, a.Server))
	})
	var commits []string
	for _, r := range reports {
		if !slices.Contains(commits, r.Commit) {
			commits = append(commits, r.Commit)
		}
	}
	slices.Sort(commits)
	fmt.Printf("Lab commit of these runs: %s.\n\n", strings.Join(commits, ", "))
	fmt.Println("| Profile | Seed | Server | Status | Prompt confirmed | First text | Max lag | Longest silence | Shown/produced/total | Wire down/up (B) | Reply received/duplicates (B) |")
	fmt.Println("|---|---|---|---|---|---|---|---|---|---|---|")
	for _, r := range reports {
		fmt.Printf("| %s | %d | [%s](%s/report.md) | %s | %s | %s | %s | %s | %d/%d/%d | %d/%d | %d/%d |\n",
			r.Profile, r.Seed, r.Server, ServerRunName(r.Profile, r.Server, r.Seed), r.Status,
			fmtSecs(r.PromptConfirmationS), fmtSecs(r.TimeToFirstTextS), fmtLag(r.MaxLagS), fmtSilence(r.LongestSilenceS),
			r.ShownChars, r.ProducedChars, r.ReplyChars, r.WireDownBytes, r.WireUpBytes,
			r.ReplyBytesReceived, r.DuplicateReplyBytes)
	}
	fmt.Println()
	fmt.Println("| Server | Runs | Done | Prompt confirmed | Reply chars shown | Wire down (B) | Reply received/duplicates (B) |")
	fmt.Println("|---|---|---|---|---|---|---|")
	for _, server := range []string{"v0.1.7", "fork"} {
		var runs, done, confirmed, shown int
		var wire uint64
		var received, duplicates int64
		for _, r := range reports {
			if r.Server != server {
				continue
			}
			runs++
			if r.Status == StatusDone {
				done++
			}
			if r.PromptConfirmationS != nil {
				confirmed++
			}
			shown += r.ShownChars
			wire += r.WireDownBytes
			received += r.ReplyBytesReceived
			duplicates += r.DuplicateReplyBytes
		}
		fmt.Printf("| %s | %d | %d | %d | %d | %d | %d/%d |\n", server, runs, done, confirmed, shown, wire, received, duplicates)
	}
	return nil
}
