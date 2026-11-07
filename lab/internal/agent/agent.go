// Package agent is the lab's scripted fake agent. The forked t3 server runs it
// as a local ACP agent (JSON-RPC over stdio). It streams a fixed reply at a
// set rate and logs when it produced each character, so the lab can measure
// how old the text is when the phone shows it.
package agent

import (
	"bufio"
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strings"
	"sync"
	"time"
)

// Model is the only model the agent offers.
const Model = "scripted"

// Config sets how the agent streams.
type Config struct {
	// CharsPerSecond is the rate at which reply text is produced.
	CharsPerSecond int
	// SoakParts makes the measured reply longer; see Script.
	SoakParts int
	// Log receives one JSON line per event. See LogEvent.
	Log io.Writer
}

// LogEvent is one line of the production log.
type LogEvent struct {
	// T is the Unix time in milliseconds.
	T int64 `json:"t"`
	// Ev is prompt, text, approval-asked, approval-answered or reply-done.
	Ev string `json:"ev"`
	// Prompt is the prompt text, for prompt events.
	Prompt string `json:"prompt,omitempty"`
	// N is the normalized length of the reply produced so far, for text events.
	N int `json:"n,omitempty"`
}

const tick = 100 * time.Millisecond

type message struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id,omitempty"`
	Method  string          `json:"method,omitempty"`
	Params  json.RawMessage `json:"params,omitempty"`
	Result  json.RawMessage `json:"result,omitempty"`
	Error   *rpcError       `json:"error,omitempty"`
}

type rpcError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

type server struct {
	cfg Config

	outMu sync.Mutex
	out   *json.Encoder
	logMu sync.Mutex

	mu       sync.Mutex
	nextID   int
	pending  map[string]chan json.RawMessage
	cancels  map[string]context.CancelFunc
	sessions int
}

// Serve runs the agent until in ends.
func Serve(ctx context.Context, cfg Config, in io.Reader, out io.Writer) error {
	if cfg.CharsPerSecond <= 0 {
		return fmt.Errorf("chars per second must be positive, got %d", cfg.CharsPerSecond)
	}
	s := &server{
		cfg:     cfg,
		out:     json.NewEncoder(out),
		pending: map[string]chan json.RawMessage{},
		cancels: map[string]context.CancelFunc{},
	}
	ctx, cancel := context.WithCancel(ctx)
	var wg sync.WaitGroup
	// When input ends, no answer can arrive: stop open turns, then wait.
	defer func() {
		cancel()
		wg.Wait()
	}()
	scanner := bufio.NewScanner(in)
	scanner.Buffer(make([]byte, 1<<20), 64<<20)
	for scanner.Scan() {
		var msg message
		if err := json.Unmarshal(scanner.Bytes(), &msg); err != nil {
			fmt.Fprintf(os.Stderr, "fake agent: ignoring a line that is not JSON: %v\n", err)
			continue
		}
		switch {
		case msg.Method == "" && msg.ID != nil:
			s.answer(msg)
		case msg.ID == nil:
			s.notification(msg)
		default:
			wg.Add(1)
			go func() {
				defer wg.Done()
				s.request(ctx, msg)
			}()
		}
	}
	return scanner.Err()
}

func (s *server) send(msg message) {
	msg.JSONRPC = "2.0"
	s.outMu.Lock()
	defer s.outMu.Unlock()
	if err := s.out.Encode(msg); err != nil {
		fmt.Fprintf(os.Stderr, "fake agent: write failed: %v\n", err)
	}
}

func (s *server) log(ev LogEvent) {
	ev.T = time.Now().UnixMilli()
	line, err := json.Marshal(ev)
	if err != nil {
		fmt.Fprintf(os.Stderr, "fake agent: log encode failed: %v\n", err)
		return
	}
	s.logMu.Lock()
	defer s.logMu.Unlock()
	if _, err := s.cfg.Log.Write(append(line, '\n')); err != nil {
		fmt.Fprintf(os.Stderr, "fake agent: log write failed: %v\n", err)
	}
}

// answer routes a client's response to one of the agent's own requests.
func (s *server) answer(msg message) {
	s.mu.Lock()
	ch, ok := s.pending[string(msg.ID)]
	delete(s.pending, string(msg.ID))
	s.mu.Unlock()
	if ok {
		ch <- msg.Result
	}
}

func (s *server) notification(msg message) {
	if msg.Method != "session/cancel" {
		return
	}
	var p struct {
		SessionID string `json:"sessionId"`
	}
	if err := json.Unmarshal(msg.Params, &p); err != nil {
		return
	}
	s.mu.Lock()
	cancel := s.cancels[p.SessionID]
	s.mu.Unlock()
	if cancel != nil {
		cancel()
	}
}

func (s *server) request(ctx context.Context, msg message) {
	result, err := s.handle(ctx, msg)
	if err != nil {
		s.send(message{ID: msg.ID, Error: &rpcError{Code: -32603, Message: err.Error()}})
		return
	}
	raw, err := json.Marshal(result)
	if err != nil {
		s.send(message{ID: msg.ID, Error: &rpcError{Code: -32603, Message: err.Error()}})
		return
	}
	s.send(message{ID: msg.ID, Result: raw})
}

func (s *server) handle(ctx context.Context, msg message) (any, error) {
	var p struct {
		SessionID string `json:"sessionId"`
		Prompt    []struct {
			Type string `json:"type"`
			Text string `json:"text"`
		} `json:"prompt"`
	}
	if len(msg.Params) > 0 {
		if err := json.Unmarshal(msg.Params, &p); err != nil {
			return nil, fmt.Errorf("bad params for %s: %w", msg.Method, err)
		}
	}
	models := map[string]any{
		"currentModelId":  Model,
		"availableModels": []any{map[string]any{"modelId": Model, "name": "Scripted reply"}},
	}
	switch msg.Method {
	case "initialize":
		return map[string]any{
			"protocolVersion": 1,
			"agentCapabilities": map[string]any{
				"loadSession":        true,
				"promptCapabilities": map[string]any{"image": true, "audio": false, "embeddedContext": true},
			},
			"authMethods": []any{},
			"agentInfo":   map[string]any{"name": "t3lab-fake-agent", "version": "1"},
		}, nil
	case "session/new":
		s.mu.Lock()
		s.sessions++
		id := fmt.Sprintf("lab-%d-%d", os.Getpid(), s.sessions)
		s.mu.Unlock()
		return map[string]any{"sessionId": id, "models": models}, nil
	case "session/load":
		return map[string]any{"models": models}, nil
	case "authenticate", "session/set_model", "session/set_mode", "session/close":
		return map[string]any{}, nil
	case "session/prompt":
		var text strings.Builder
		for _, block := range p.Prompt {
			if block.Type == "text" {
				text.WriteString(block.Text)
			}
		}
		turnCtx, cancel := context.WithCancel(ctx)
		s.mu.Lock()
		s.cancels[p.SessionID] = cancel
		s.mu.Unlock()
		defer func() {
			cancel()
			s.mu.Lock()
			delete(s.cancels, p.SessionID)
			s.mu.Unlock()
		}()
		s.log(LogEvent{Ev: "prompt", Prompt: text.String()})
		stop := s.play(turnCtx, p.SessionID, Script(text.String(), s.cfg.SoakParts))
		return map[string]any{"stopReason": stop}, nil
	}
	return nil, fmt.Errorf("method not found: %s", msg.Method)
}

func (s *server) update(sessionID string, update map[string]any) {
	params, err := json.Marshal(map[string]any{"sessionId": sessionID, "update": update})
	if err != nil {
		fmt.Fprintf(os.Stderr, "fake agent: encode update: %v\n", err)
		return
	}
	s.send(message{Method: "session/update", Params: params})
}

// play streams the steps and returns the ACP stop reason.
func (s *server) play(ctx context.Context, sessionID string, steps []Step) string {
	chunk := max(s.cfg.CharsPerSecond*int(tick)/int(time.Second), 1)
	produced := 0
	ticker := time.NewTicker(tick)
	defer ticker.Stop()
	for i, step := range steps {
		switch {
		case step.Text != "":
			text := step.Text
			for len(text) > 0 {
				select {
				case <-ctx.Done():
					return "cancelled"
				case <-ticker.C:
				}
				n := min(chunk, len(text))
				part := text[:n]
				text = text[n:]
				s.update(sessionID, map[string]any{
					"sessionUpdate": "agent_message_chunk",
					"content":       map[string]any{"type": "text", "text": part},
				})
				produced += len(Normalize(part))
				s.log(LogEvent{Ev: "text", N: produced})
			}
		case step.Approval != "":
			if !s.approve(ctx, sessionID, fmt.Sprintf("lab-tool-%d", i), step.Approval) {
				return "cancelled"
			}
		case step.Image != nil:
			s.update(sessionID, map[string]any{
				"sessionUpdate": "agent_message_chunk",
				"content": map[string]any{
					"type":     "image",
					"mimeType": "image/png",
					"data":     base64.StdEncoding.EncodeToString(step.Image),
				},
			})
		}
	}
	s.log(LogEvent{Ev: "reply-done"})
	return "end_turn"
}

// approve asks the client for permission and waits for the answer. It reports
// false when the turn was cancelled or the permission was not granted.
func (s *server) approve(ctx context.Context, sessionID, toolCallID, title string) bool {
	s.update(sessionID, map[string]any{
		"sessionUpdate": "tool_call",
		"toolCallId":    toolCallID,
		"kind":          "edit",
		"title":         title,
		"status":        "pending",
	})
	s.mu.Lock()
	s.nextID++
	id := json.RawMessage(fmt.Sprintf("%d", s.nextID))
	ch := make(chan json.RawMessage, 1)
	s.pending[string(id)] = ch
	s.mu.Unlock()
	params, err := json.Marshal(map[string]any{
		"sessionId": sessionID,
		"toolCall":  map[string]any{"toolCallId": toolCallID, "kind": "edit", "title": title},
		"options": []any{
			map[string]any{"optionId": "allow-once", "name": "Yes", "kind": "allow_once"},
			map[string]any{"optionId": "reject-once", "name": "No", "kind": "reject_once"},
		},
	})
	if err != nil {
		fmt.Fprintf(os.Stderr, "fake agent: encode permission request: %v\n", err)
		return false
	}
	s.log(LogEvent{Ev: "approval-asked"})
	s.send(message{ID: id, Method: "session/request_permission", Params: params})
	var result json.RawMessage
	select {
	case <-ctx.Done():
		s.mu.Lock()
		delete(s.pending, string(id))
		s.mu.Unlock()
		return false
	case result = <-ch:
	}
	var answer struct {
		Outcome struct {
			Outcome  string `json:"outcome"`
			OptionID string `json:"optionId"`
		} `json:"outcome"`
	}
	if err := json.Unmarshal(result, &answer); err != nil {
		fmt.Fprintf(os.Stderr, "fake agent: bad permission answer: %v\n", err)
		return false
	}
	s.log(LogEvent{Ev: "approval-answered"})
	allowed := answer.Outcome.Outcome == "selected" && answer.Outcome.OptionID == "allow-once"
	status := "completed"
	if !allowed {
		status = "failed"
	}
	s.update(sessionID, map[string]any{"sessionUpdate": "tool_call_update", "toolCallId": toolCallID, "status": status})
	return allowed
}
