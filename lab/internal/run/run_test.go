package run

import (
	"os"
	"strings"
	"testing"
)

func TestBaselineCommitMatchesBuildScript(t *testing.T) {
	script, err := os.ReadFile("../../scripts/build-baseline-client.sh")
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(string(script), "commit="+BaselineCommit+"\n") {
		t.Fatalf("build-baseline-client.sh does not build commit %s", BaselineCommit)
	}
}
