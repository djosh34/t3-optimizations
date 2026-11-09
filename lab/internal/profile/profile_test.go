package profile

import (
	"slices"
	"strings"
	"testing"
	"time"
)

func TestNetemArgs(t *testing.T) {
	cases := map[string]string{
		"case70": "limit 20 delay 300ms 50ms distribution normal loss random 70% rate 10000bit seed 7",
		"F":      "limit 2 delay 2000ms 500ms distribution normal loss gemodel 15% 5% 100% 0% rate 64bit seed 7",
		"reorder": "limit 50 delay 300ms 50ms distribution normal loss random 10% reorder 25% 50% " +
			"rate 100000bit seed 7",
		"rate100": "limit 4 delay 300ms 50ms distribution normal rate 800bit seed 7",
	}
	for name, want := range cases {
		p, err := Get(name)
		if err != nil {
			t.Fatal(err)
		}
		if got := strings.Join(p.Down.NetemArgs(7), " "); got != want {
			t.Errorf("%s:\n got %s\nwant %s", name, got, want)
		}
	}
}

func TestOutageAt(t *testing.T) {
	flap := Outages{First: 10 * time.Second, Down: 5 * time.Second, Period: 15 * time.Second}
	once := Outages{First: 10 * time.Second, Down: 60 * time.Second}
	for _, c := range []struct {
		o    Outages
		at   time.Duration
		down bool
	}{
		{flap, 9 * time.Second, false},
		{flap, 10 * time.Second, true},
		{flap, 14 * time.Second, true},
		{flap, 15 * time.Second, false},
		{flap, 26 * time.Second, true},
		{once, 69 * time.Second, true},
		{once, 70 * time.Second, false},
		{once, 100 * time.Second, false},
		{Outages{}, 5 * time.Second, false},
	} {
		if got := c.o.OutageAt(c.at); got != c.down {
			t.Errorf("%+v at %v: got %v", c.o, c.at, got)
		}
	}
}

func TestProfiles(t *testing.T) {
	var names []string
	for _, p := range All {
		if slices.Contains(names, p.Name) {
			t.Errorf("profile %s listed twice", p.Name)
		}
		names = append(names, p.Name)
		if p.Down.RateBits == 0 || p.Up.RateBits == 0 || p.Down.Limit == 0 || p.Up.Limit == 0 {
			t.Errorf("profile %s has no rate or queue", p.Name)
		}
	}
	for _, name := range []string{"A", "B", "C", "D", "E", "case70", "case50"} {
		if p, _ := Get(name); p.Grade() != "graded" {
			t.Errorf("%s should be graded", name)
		}
	}
	if p, _ := Get("F"); p.Grade() != "diagnostic" {
		t.Error("F should be diagnostic")
	}
}

func TestFromStory(t *testing.T) {
	data := []byte(`{"seed":3,"survival":false,"runMs":300000,"segments":[
		{"atMs":0,"name":"bursts","down":{"rateBps":96000,"delayMs":600,"jitterMs":40,"queue":20,"loss":0,"burst":[0.02,0.25]},
		 "up":{"rateBps":48000,"delayMs":600,"jitterMs":40,"queue":10,"loss":0,"burst":[0.02,0.25]}},
		{"atMs":4000,"name":"gap","down":{"rateBps":96000,"delayMs":600,"jitterMs":40,"queue":20,"loss":1,"burst":null},
		 "up":{"rateBps":48000,"delayMs":600,"jitterMs":40,"queue":10,"loss":1,"burst":null}}]}`)
	p, err := FromStory(data)
	if err != nil {
		t.Fatal(err)
	}
	if len(p.Phases) != 2 || p.Phases[1].At != 4*time.Second || p.Deadline() != 5*time.Minute {
		t.Fatalf("phases %+v, deadline %s", p.Phases, p.Deadline())
	}
	want := "limit 28 delay 600ms 40ms distribution normal loss gemodel 2.00% 25.00% 100% 0% rate 96000bit seed 1"
	if got := strings.Join(p.Down.NetemArgs(1), " "); got != want {
		t.Errorf("down netem\n got %s\nwant %s", got, want)
	}
	if got := strings.Join(p.Phases[1].Up.Loss, " "); got != "random 100.00%" {
		t.Errorf("gap loss %q", got)
	}
}
