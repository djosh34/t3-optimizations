package report

import (
	"strings"
	"testing"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
)

// A run where the agent produces 100 characters over 10 s, the phone shows
// the first 50 after 3 s, then nothing until 20 s, when it shows the rest.
func TestBuild(t *testing.T) {
	base := time.UnixMilli(1_000_000)
	ms := func(d time.Duration) int64 { return base.Add(d).UnixMilli() }
	var log []agent.LogEvent
	log = append(log, agent.LogEvent{T: ms(0), Ev: "prompt", Prompt: "<t3_code_instructions>…</t3_code_instructions>" + agent.MeasuredPrompt})
	for i := 1; i <= 10; i++ {
		log = append(log, agent.LogEvent{T: ms(time.Duration(i) * time.Second), Ev: "text", N: i * 10})
	}
	events := []phone.Event{
		{T: ms(0), Ev: "prompt-sent"},
		{T: ms(500 * time.Millisecond), Ev: "prompt-confirmed"},
		{T: ms(3 * time.Second), Ev: "text", N: 20},
		{T: ms(6 * time.Second), Ev: "text", N: 50},
		{T: ms(20 * time.Second), Ev: "text", N: 100},
		{T: ms(20 * time.Second), Ev: "complete"},
		{T: ms(21 * time.Second), Ev: "stats", Stats: phone.Stats{ReplyBytes: 150, DuplicateReplyBytes: 50}},
	}
	r := Build(Input{
		Profile: "case50", Grade: "graded", Client: "websocket", Kind: "warm",
		Switched: base.Add(-time.Second), End: base.Add(20 * time.Second),
		Reply: strings.Repeat("a", 100), Agent: log, Phone: events,
	})
	if r.Status != StatusDone {
		t.Fatalf("status %q", r.Status)
	}
	check := func(name string, got *float64, want float64) {
		t.Helper()
		if got == nil || *got != want {
			t.Errorf("%s = %s, want %v", name, fmtSecs(got), want)
		}
	}
	check("prompt confirmation", r.PromptConfirmationS, 0.5)
	check("first text", r.TimeToFirstTextS, 3)
	check("complete", r.CompleteS, 20)
	check("prompt reached the agent", r.AgentGotPromptS, 0)
	// After 6 s the phone shows 50; character 51 was produced at 6 s. At 19 s
	// it is 13 s old.
	check("max lag", r.MaxLagS, 13)
	// From 6 s, when the phone showed 50, to 20 s.
	check("longest silence", r.LongestSilenceS, 14)
	if r.DuplicateReplyBytes != 50 || r.ProducedChars != 100 || r.ShownChars != 100 {
		t.Errorf("duplicates %d, produced %d, shown %d", r.DuplicateReplyBytes, r.ProducedChars, r.ShownChars)
	}
}

func TestBuildNothingShown(t *testing.T) {
	base := time.UnixMilli(1_000_000)
	r := Build(Input{
		Kind:     "warm",
		Switched: base, End: base.Add(15 * time.Minute),
		Reply: "abc",
		Agent: []agent.LogEvent{
			{T: base.UnixMilli(), Ev: "prompt", Prompt: agent.MeasuredPrompt},
			{T: base.Add(time.Second).UnixMilli(), Ev: "text", N: 3},
		},
		Phone: []phone.Event{{T: base.UnixMilli(), Ev: "prompt-sent"}},
	})
	if r.Status != StatusNotDone || r.TimeToFirstTextS != nil || r.PromptConfirmationS != nil {
		t.Fatalf("got %+v", r)
	}
	if r.LongestSilenceS == nil || *r.LongestSilenceS != 900 {
		t.Errorf("longest silence %s, want 900", fmtSecs(r.LongestSilenceS))
	}
	if r.MaxLagS == nil || *r.MaxLagS != 899 {
		t.Errorf("max lag %s, want 899", fmtSecs(r.MaxLagS))
	}
}

// The prompt never reached the agent: the phone showed nothing for the
// whole run, and there is no lag to report.
func TestBuildPromptLost(t *testing.T) {
	base := time.UnixMilli(1_000_000)
	r := Build(Input{
		Kind:     "warm",
		Switched: base, End: base.Add(15 * time.Minute),
		Reply: "abc",
		Phone: []phone.Event{{T: base.UnixMilli(), Ev: "prompt-sent"}},
	})
	if r.MaxLagS != nil || r.AgentGotPromptS != nil || r.LongestSilenceS == nil || *r.LongestSilenceS != 900 {
		t.Fatalf("max lag %s, agent got prompt %s, longest silence %s", fmtSecs(r.MaxLagS), fmtSecs(r.AgentGotPromptS), fmtSecs(r.LongestSilenceS))
	}
}

func TestBuildColdNeverLoaded(t *testing.T) {
	base := time.UnixMilli(1_000_000)
	r := Build(Input{Kind: "cold", Switched: base, End: base.Add(time.Minute), Reply: "abc"})
	if r.Status != StatusNeverLoaded || r.LongestSilenceS != nil {
		t.Fatalf("status %q, longest silence %s", r.Status, fmtSecs(r.LongestSilenceS))
	}
}
