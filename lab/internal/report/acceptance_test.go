package report

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestBackfillIsReadAtTheMomentsAfterThePrompt(t *testing.T) {
	dir := t.TempDir()
	events := `{"t":1000,"ev":"prompt-sent"}
{"t":2500,"ev":"prompt-confirmed"}
`
	log := `sidecar ready
sidecar: 500 device d1: command 3 (message.dispatch) replied, backfill unsent 9000 B
sidecar: 1200 device d1: command 4 (thread.visit) replied, backfill unsent 800 B
sidecar: 1500 device d1: command 5 (message.dispatch) replied, backfill unsent 700 B
sidecar: 1600 device d1: new newest text, backfill unsent 650 B
sidecar: 3000 device d1: command 6 (message.dispatch) replied, backfill unsent 0 B
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
	if atConfirm != 700 || atText != 650 {
		t.Errorf("got %d and %d, want 700 and 650", atConfirm, atText)
	}
}

func TestBackfillNeedsTheConfirmationAndTextOnThePhone(t *testing.T) {
	confirmed := 2.0
	queued := accepted{Report: Report{Profile: "case50", Client: "webapp", Kind: "warm", Seed: 1}, atConfirm: 700, atText: 650}
	out := targets([]accepted{queued})
	if !strings.Contains(out, "overtake backfill in every run: **not met**") {
		t.Errorf("queued but never delivered counted:\n%s", out)
	}
	queued.PromptConfirmationS, queued.TimeToFirstTextS = &confirmed, &confirmed
	out = targets([]accepted{queued})
	if !strings.Contains(out, "overtake backfill in every run: **met**") {
		t.Errorf("queued ahead of backfill and delivered not counted:\n%s", out)
	}
}

func TestTargetsSayWhichAreMet(t *testing.T) {
	fast, slow := 3.0, 12.0
	runs := []accepted{
		{Report: Report{Profile: "case70", Client: "webapp", Kind: "warm", Seed: 1, PromptConfirmationS: &fast}, atConfirm: -1, atText: -1},
		{Report: Report{Profile: "case70", Client: "webapp", Kind: "warm", Seed: 2, PromptConfirmationS: &slow,
			Notes: []string{"the prompt is still in the composer: Enter again"}}, atConfirm: -1, atText: -1},
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
