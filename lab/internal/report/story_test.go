package report

import (
	"testing"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
)

func TestReleasedSamplesFollowTheServer(t *testing.T) {
	reply := agent.Normalize("The first paragraph is here. The second one follows it.")
	t0 := time.UnixMilli(1_000_000)
	rel := []Release{
		// A setup turn's message, there before the prompt: not the reply.
		{T: t0.UnixMilli() - 500, Thread: "a", Item: "old", Ordinal: 1, Text: "Ready."},
		{T: t0.UnixMilli() + 2_000, Thread: "a", Item: "r", Ordinal: 2, Text: "The first paragraph is here.\n\n"},
		{T: t0.UnixMilli() + 3_000, Thread: "b", Item: "x", Ordinal: 2, Text: "The first paragraph is here. The second one follows it."},
		{T: t0.UnixMilli() + 9_000, Thread: "a", Item: "r", Ordinal: 2, Text: "The first paragraph is here.\n\nThe second one follows it."},
	}
	got := releasedSamples(rel, "a", t0, reply)
	want := []sample{{t0.Add(2 * time.Second), len(agent.Normalize("The first paragraph is here."))}, {t0.Add(9 * time.Second), len(reply)}}
	if len(got) != len(want) {
		t.Fatalf("got %v, want %v", got, want)
	}
	for i := range want {
		if !got[i].t.Equal(want[i].t) || got[i].n != want[i].n {
			t.Errorf("sample %d: got %v, want %v", i, got[i], want[i])
		}
	}
}

func TestFreezesCountOnlyWhileTheServerIsAhead(t *testing.T) {
	t0 := time.UnixMilli(1_000_000)
	at := func(s float64) time.Time { return t0.Add(time.Duration(s * float64(time.Second))) }
	released := []sample{{at(5), 100}, {at(60), 200}}
	shown := []sample{{at(10), 100}, {at(100), 200}}
	got := freezes(t0, at(120), shown, released)
	// Behind from 5 s to 10 s, then from 60 s until 100 s.
	if len(got) != 2 || got[0].ForS != 5 || got[1].ForS != 40 || got[1].AtS != 60 {
		t.Fatalf("freezes %+v", got)
	}
}

func TestStoryClientGrades(t *testing.T) {
	t0 := time.UnixMilli(1_000_000)
	reply := agent.Normalize("The first paragraph is here.")
	ev := func(s float64, name string, n int) phone.Event {
		return phone.Event{T: t0.Add(time.Duration(s * float64(time.Second))).UnixMilli(), Ev: name, N: n}
	}
	in := Input{
		Client: "webapp", Kind: "warm", Switched: t0, End: t0.Add(time.Minute), Reply: reply,
		Phone: []phone.Event{
			ev(0, "prompt-sent", 0), ev(3, "prompt-confirmed", 0), ev(12, "text", len(reply)), ev(12, "complete", 0),
			{T: t0.UnixMilli() + 1, Ev: "transport", Name: "webrtc", N: 1},
		},
	}
	rel := []Release{{T: t0.UnixMilli() + 10_000, Thread: "a", Item: "r", Ordinal: 1, Text: "The first paragraph is here."}}
	c := BuildStoryClient(in, rel, "a")
	if !c.PromptMet() || !c.LagMet() || !c.OnUDP() || *c.MaxReleaseLagS != 2 {
		t.Fatalf("%+v, lag %v", c, *c.MaxReleaseLagS)
	}
}
