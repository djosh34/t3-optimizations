package agent

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"io"
	"strings"
	"sync"
	"testing"
	"time"
)

type lockedBuffer struct {
	mu  sync.Mutex
	buf bytes.Buffer
}

func (b *lockedBuffer) Write(p []byte) (int, error) {
	b.mu.Lock()
	defer b.mu.Unlock()
	return b.buf.Write(p)
}

func (b *lockedBuffer) String() string {
	b.mu.Lock()
	defer b.mu.Unlock()
	return b.buf.String()
}

// TestMeasuredTurn plays the measured reply through the ACP protocol: the text
// arrives in order, the approval is asked once and answered, and the log
// counts every normalized character.
func TestMeasuredTurn(t *testing.T) {
	clientToAgent, agentIn := io.Pipe()
	agentOut, agentToClient := io.Pipe()
	var log lockedBuffer
	done := make(chan error, 1)
	go func() {
		done <- Serve(context.Background(), Config{CharsPerSecond: 1_000_000, Log: &log}, clientToAgent, agentToClient)
		agentToClient.Close()
	}()
	enc := json.NewEncoder(agentIn)
	send := func(v map[string]any) {
		v["jsonrpc"] = "2.0"
		if err := enc.Encode(v); err != nil {
			t.Fatalf("send: %v", err)
		}
	}
	send(map[string]any{"id": 1, "method": "initialize", "params": map[string]any{}})
	send(map[string]any{"id": 2, "method": "session/new", "params": map[string]any{"cwd": "/tmp"}})

	lines := bufio.NewScanner(agentOut)
	lines.Buffer(make([]byte, 1<<20), 1<<24)
	var sessionID string
	var text strings.Builder
	images, approvals := 0, 0
	for lines.Scan() {
		var msg struct {
			ID     json.RawMessage `json:"id"`
			Method string          `json:"method"`
			Params struct {
				Update struct {
					SessionUpdate string `json:"sessionUpdate"`
					Content       struct {
						Type string `json:"type"`
						Text string `json:"text"`
					} `json:"content"`
				} `json:"update"`
			} `json:"params"`
			Result struct {
				SessionID  string `json:"sessionId"`
				StopReason string `json:"stopReason"`
			} `json:"result"`
		}
		if err := json.Unmarshal(lines.Bytes(), &msg); err != nil {
			t.Fatalf("agent wrote invalid JSON %q: %v", lines.Text(), err)
		}
		switch {
		case string(msg.ID) == "2":
			sessionID = msg.Result.SessionID
			send(map[string]any{"id": 3, "method": "session/prompt", "params": map[string]any{
				"sessionId": sessionID,
				"prompt":    []any{map[string]any{"type": "text", "text": MeasuredPrompt}},
			}})
		case msg.Method == "session/request_permission":
			approvals++
			send(map[string]any{"id": json.RawMessage(msg.ID), "result": map[string]any{
				"outcome": map[string]any{"outcome": "selected", "optionId": "allow-once"},
			}})
		case msg.Method == "session/update" && msg.Params.Update.SessionUpdate == "agent_message_chunk":
			if msg.Params.Update.Content.Type == "image" {
				images++
			} else {
				text.WriteString(msg.Params.Update.Content.Text)
			}
		case string(msg.ID) == "3":
			if msg.Result.StopReason != "end_turn" {
				t.Fatalf("stop reason %q", msg.Result.StopReason)
			}
			agentIn.Close()
		}
	}
	if err := <-done; err != nil {
		t.Fatalf("serve: %v", err)
	}
	want := ReplyText(Script(MeasuredPrompt, 0))
	if text.String() != want {
		t.Fatalf("streamed text differs from the script")
	}
	if approvals != 1 || images != 1 {
		t.Fatalf("approvals %d, images %d, want 1 and 1", approvals, images)
	}
	events := strings.Split(strings.TrimSpace(log.String()), "\n")
	var last, prev LogEvent
	if err := json.Unmarshal([]byte(events[len(events)-1]), &last); err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal([]byte(events[len(events)-2]), &prev); err != nil {
		t.Fatal(err)
	}
	if last.Ev != "reply-done" || prev.N != len(Normalize(want)) {
		t.Fatalf("log ends with %+v after %+v, want reply-done after n=%d", last, prev, len(Normalize(want)))
	}
}

func TestReplyIsASCII(t *testing.T) {
	for _, r := range ReplyText(Script(MeasuredPrompt, 3)) {
		if r > 127 {
			t.Fatalf("reply has non-ASCII rune %q; the agent splits text by bytes", r)
		}
	}
}

// When the server goes away while the agent waits for an approval, the agent
// must exit instead of waiting for an answer that cannot come.
func TestEOFDuringApproval(t *testing.T) {
	clientToAgent, agentIn := io.Pipe()
	agentOut, agentToClient := io.Pipe()
	done := make(chan error, 1)
	go func() {
		done <- Serve(context.Background(), Config{CharsPerSecond: 1_000_000, Log: io.Discard}, clientToAgent, agentToClient)
	}()
	enc := json.NewEncoder(agentIn)
	for _, msg := range []map[string]any{
		{"jsonrpc": "2.0", "id": 1, "method": "session/new", "params": map[string]any{}},
		{"jsonrpc": "2.0", "id": 2, "method": "session/prompt", "params": map[string]any{
			"sessionId": "s", "prompt": []any{map[string]any{"type": "text", "text": MeasuredPrompt}},
		}},
	} {
		if err := enc.Encode(msg); err != nil {
			t.Fatal(err)
		}
	}
	lines := bufio.NewScanner(agentOut)
	lines.Buffer(make([]byte, 1<<20), 1<<24)
	for lines.Scan() {
		if strings.Contains(lines.Text(), "session/request_permission") {
			break
		}
	}
	go func() {
		for lines.Scan() {
		}
	}()
	agentIn.Close()
	select {
	case err := <-done:
		if err != nil {
			t.Fatalf("serve: %v", err)
		}
	case <-time.After(5 * time.Second):
		t.Fatal("agent still waiting for the approval after its input closed")
	}
}
