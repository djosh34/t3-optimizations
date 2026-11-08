package report

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestBackfillTakesTheLastStatsLineBeforeEachEvent(t *testing.T) {
	dir := t.TempDir()
	events := `{"t":1000,"ev":"prompt-sent"}
{"t":2500,"ev":"prompt-confirmed"}
{"t":4500,"ev":"text","n":12}
`
	log := `sidecar ready
sidecar: 2000 device d1 stats: send 2000 B/s, unsynced 900 B, in flight 3 packets
sidecar: 4000 device d1 stats: send 2000 B/s, unsynced 400 B, in flight 3 packets
sidecar: 6000 device d1 stats: send 2000 B/s, unsynced 0 B, in flight 0 packets
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
	if atConfirm != 900 || atText != 400 {
		t.Errorf("got %d at the confirmation and %d at the first text, want 900 and 400", atConfirm, atText)
	}
}

func TestTargetsSayWhichAreMet(t *testing.T) {
	fast, slow := 3.0, 12.0
	runs := []accepted{
		{Report: Report{Profile: "case70", Client: "webapp", Kind: "warm", Seed: 1, PromptConfirmationS: &fast}, backfillAtConfirm: -1, backfillAtText: -1},
		{Report: Report{Profile: "case70", Client: "webapp", Kind: "warm", Seed: 2, PromptConfirmationS: &slow}, backfillAtConfirm: -1, backfillAtText: -1},
	}
	out := targets(runs)
	if !strings.Contains(out, "within 10 s: **not met**. case70-webapp-warm 3.0s, case70-webapp-warm-s2 12.0s.") {
		t.Errorf("case 70 line wrong:\n%s", out)
	}
	if !strings.Contains(out, "twice, over all 2 runs: 0") {
		t.Errorf("duplicates line wrong:\n%s", out)
	}
}
