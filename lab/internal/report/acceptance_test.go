package report

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestBackfillIsReadAroundEachMoment(t *testing.T) {
	dir := t.TempDir()
	events := `{"t":1000,"ev":"prompt-sent"}
{"t":2500,"ev":"prompt-confirmed"}
{"t":4500,"ev":"text","n":12}
`
	log := `sidecar ready
sidecar: 2000 device d1 stats: send 2000 B/s, unsynced 1200 B, backfill unsent 900 B, in flight 3 packets
sidecar: 4000 device d1 stats: send 2000 B/s, unsynced 600 B, backfill unsent 400 B, in flight 3 packets
sidecar: 6000 device d1 stats: send 2000 B/s, unsynced 0 B, backfill unsent 0 B, in flight 0 packets
`
	for name, data := range map[string]string{"phone.jsonl": events, "sidecar.log": log} {
		if err := os.WriteFile(filepath.Join(dir, name), []byte(data), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	atConfirm, atText, err := backfill(dir)
	if err != nil {
		t.Fatal(err)
	}
	if atConfirm != (around{900, 400}) || atText != (around{400, 0}) {
		t.Errorf("got %v and %v, want 900 → 400 and 400 → 0", atConfirm, atText)
	}
	if !atConfirm.overtook() {
		t.Errorf("the confirmation overtook backfill")
	}
	if atText.overtook() {
		t.Errorf("backfill drained between the lines around the first text: cannot be told")
	}
}

func TestTargetsSayWhichAreMet(t *testing.T) {
	fast, slow := 3.0, 12.0
	runs := []accepted{
		{Report: Report{Profile: "case70", Client: "webapp", Kind: "warm", Seed: 1, PromptConfirmationS: &fast}, atConfirm: around{-1, -1}, atText: around{-1, -1}},
		{Report: Report{Profile: "case70", Client: "webapp", Kind: "warm", Seed: 2, PromptConfirmationS: &slow,
			Notes: []string{"the prompt is still in the composer: Enter again"}}, atConfirm: around{-1, -1}, atText: around{-1, -1}},
	}
	out := targets(runs)
	if !strings.Contains(out, "within 10 s: **not met**. case70-webapp-warm 3.0s, case70-webapp-warm-s2 12.0s.") {
		t.Errorf("case 70 line wrong:\n%s", out)
	}
	if !strings.Contains(out, "pressed it again, in 1 of 2 runs: case70-webapp-warm-s2 1 more.") {
		t.Errorf("Enter line wrong:\n%s", out)
	}
	if !strings.Contains(out, "twice, over all 2 runs: 0") {
		t.Errorf("duplicates line wrong:\n%s", out)
	}
}
