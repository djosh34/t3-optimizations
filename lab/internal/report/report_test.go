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
			t.Errorf("%s = %v, want %v", name, got, want)
		}
	}
	check("prompt confirmation", r.PromptConfirmationS, 0.5)
	check("first text", r.TimeToFirstTextS, 3)
	check("complete", r.CompleteS, 20)
	// After 6 s the phone shows 50; character 51 was produced at 6 s. At 19 s
	// it is 13 s old.
	if r.MaxLagS != 13 {
		t.Errorf("max lag %v, want 13", r.MaxLagS)
	}
	// From 6 s, when character 51 existed and the phone had just shown 50,
	// to 20 s.
	if r.LongestSilenceS != 14 {
		t.Errorf("longest silence %v, want 14", r.LongestSilenceS)
	}
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
	if r.LongestSilenceS != 899 {
		t.Errorf("longest silence %v, want 899", r.LongestSilenceS)
	}
}

func TestBuildColdNeverLoaded(t *testing.T) {
	base := time.UnixMilli(1_000_000)
	r := Build(Input{Kind: "cold", Switched: base, End: base.Add(time.Minute), Reply: "abc"})
	if r.Status != StatusNeverLoaded {
		t.Fatalf("status %q", r.Status)
	}
}
