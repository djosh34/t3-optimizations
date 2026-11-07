package pipereport

import (
	"math"
	"testing"
	"time"
)

func seq(n int64) *int64 { return &n }

func TestDirection(t *testing.T) {
	switched := time.UnixMilli(10_000)
	end := time.UnixMilli(20_000)
	sender := []Event{
		{T: 9_900, Ev: "sent", Seq: seq(0)}, // before the switch: not counted
		{T: 10_000, Ev: "sent", Seq: seq(1)},
		{T: 10_100, Ev: "sent", Seq: seq(2)},
		{T: 10_200, Ev: "sent", Seq: seq(3)},
		{T: 13_000, Ev: "sent", Seq: seq(4)},
	}
	receiver := []Event{
		// Sent before the switch, arrived after it: a gap ends, but it is
		// not one of the ticks sent after the switch.
		{T: 10_200, Ev: "recv", Seq: seq(0), Sent: 9_900},
		{T: 10_400, Ev: "recv", Seq: seq(2), Sent: 10_100},
		{T: 10_400, Ev: "recv", Seq: seq(2), Sent: 10_100}, // duplicate
		{T: 14_000, Ev: "recv", Seq: seq(4), Sent: 13_000},
		{T: 20_500, Ev: "recv", Seq: seq(3), Sent: 10_200}, // after the end
	}
	d := direction(sender, receiver, switched, end)
	if d.Sent != 4 || d.Received != 2 {
		t.Fatalf("sent/received = %d/%d, want 4/2", d.Sent, d.Received)
	}
	if d.DeliveredPct != 50 {
		t.Errorf("delivered = %v, want 50", d.DeliveredPct)
	}
	// Arrivals at 10.2, 10.4, 14.0 and the end at 20.0: the longest gap is
	// the 6 s from 14.0 to the end, starting 4 s after the switch.
	if d.LongestGapS != 6 || d.LongestGapAtS != 4 {
		t.Errorf("longest gap %v at %v, want 6 at 4", d.LongestGapS, d.LongestGapAtS)
	}
	if d.GapsOver2S != 2 {
		t.Errorf("gaps over 2 s = %d, want 2", d.GapsOver2S)
	}
	if d.AgeMaxS == nil || math.Abs(*d.AgeMaxS-1) > 1e-9 {
		t.Errorf("max age = %v, want 1", d.AgeMaxS)
	}
}

func TestDirectionNothingArrived(t *testing.T) {
	d := direction([]Event{{T: 10_000, Ev: "sent", Seq: seq(0)}}, nil, time.UnixMilli(10_000), time.UnixMilli(15_000))
	if d.Received != 0 || d.LongestGapS != 5 || d.AgeMedianS != nil {
		t.Errorf("got %+v, want nothing received, a 5 s gap and no age", d)
	}
}
