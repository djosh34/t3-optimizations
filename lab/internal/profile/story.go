package profile

import (
	"encoding/json"
	"fmt"
	"math"
	"strconv"
	"time"
)

// A story is a seed's network story as `lab/bin/story <seed>` prints it,
// from crates/sync-core/src/sim/story.rs: the one generator the lab and the
// simulator share.
type story struct {
	Seed     int64          `json:"seed"`
	Survival bool           `json:"survival"`
	RunMs    int64          `json:"runMs"`
	Segments []storySegment `json:"segments"`
}

type storySegment struct {
	AtMs int64     `json:"atMs"`
	Name string    `json:"name"`
	Down storyLink `json:"down"`
	Up   storyLink `json:"up"`
}

type storyLink struct {
	RateBps  int64       `json:"rateBps"`
	DelayMs  int64       `json:"delayMs"`
	JitterMs int64       `json:"jitterMs"`
	Queue    int         `json:"queue"`
	Loss     float64     `json:"loss"`
	Burst    *[2]float64 `json:"burst"`
}

func pct(f float64) string { return strconv.FormatFloat(f*100, 'f', 2, 64) + "%" }

// netem turns a story link into a netem link. The simulator's queue holds
// packets waiting for the rate; netem's limit also counts packets in their
// delay, so the limit adds what the delay holds at full rate.
func (l storyLink) netem() Link {
	delay := time.Duration(l.DelayMs) * time.Millisecond
	inDelay := float64(l.RateBps) / 8 / 1200 * (delay + 3*time.Duration(l.JitterMs)*time.Millisecond).Seconds()
	link := Link{
		RateBits: l.RateBps,
		Delay:    delay,
		Jitter:   time.Duration(l.JitterMs) * time.Millisecond,
		Limit:    l.Queue + int(math.Ceil(inDelay)),
	}
	switch {
	case l.Burst != nil:
		link.Loss = ge(pct(l.Burst[0]), pct(l.Burst[1]))
	case l.Loss > 0:
		link.Loss = random(pct(l.Loss))
	}
	return link
}

// FromStory makes a profile of a story's JSON: one phase per segment, the
// run's deadline at the story's end.
func FromStory(data []byte) (Profile, error) {
	var st story
	if err := json.Unmarshal(data, &st); err != nil {
		return Profile{}, fmt.Errorf("story: %w", err)
	}
	if len(st.Segments) == 0 || st.Segments[0].AtMs != 0 {
		return Profile{}, fmt.Errorf("story of seed %d: no segment at 0", st.Seed)
	}
	about := fmt.Sprintf("seed %d's 2G story", st.Seed)
	if st.Survival {
		about = fmt.Sprintf("seed %d's survival story", st.Seed)
	}
	p := Profile{Name: "story", About: about, Graded: true, RunFor: time.Duration(st.RunMs) * time.Millisecond}
	for _, seg := range st.Segments {
		p.Phases = append(p.Phases, Phase{
			At: time.Duration(seg.AtMs) * time.Millisecond, Name: seg.Name,
			Down: seg.Down.netem(), Up: seg.Up.netem(),
		})
	}
	p.Down, p.Up = p.Phases[0].Down, p.Phases[0].Up
	return p, nil
}
