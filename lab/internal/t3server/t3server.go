// Package t3server runs the lab's throwaway t3 server: the stock v0.1.7 server
// from t3code/, with its own base dir, a home dir of its own, and the fake
// agent as its only provider. It serves no web client; the lab front does.
package t3server

import (
	"bufio"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/userproc"
)

// Port is the server's port inside the server namespace.
const Port = 3780

// Config sets up the server.
type Config struct {
	User userproc.User
	// NS is the network namespace to run in.
	NS string
	// LabID names the server's temp base dir, so deleting the lab deletes it.
	LabID string
	// T3Code is the vendored t3code/ directory.
	T3Code string
	// RunDir holds the server's data, logs and the agent log.
	RunDir string
	// LabExe is the lab binary, which the server starts as the fake agent.
	LabExe string
	// Node is the node binary that runs the server.
	Node           string
	CharsPerSecond int
	SoakParts      int
}

// Server is a running t3 server.
type Server struct {
	cfg     Config
	baseDir string
	work    string
	home    string
	done    chan error
}

// AgentLog is the fake agent's production log in a run dir.
func AgentLog(runDir string) string { return filepath.Join(runDir, "agent.jsonl") }

// Start starts the server and waits until it is ready. Its base dir is a
// fresh temp dir named after the lab, which netlab deletes with the lab.
func Start(ctx context.Context, cfg Config) (*Server, error) {
	u := cfg.User
	out, err := u.Command(ctx, "", []string{"PATH=/usr/bin:/bin"}, "/", "mktemp", "-d", netlab.TempDirPattern(cfg.LabID)).Output()
	if err != nil {
		return nil, fmt.Errorf("mktemp: %w", err)
	}
	s := &Server{
		cfg:     cfg,
		baseDir: strings.TrimSpace(string(out)),
		work:    filepath.Join(cfg.RunDir, "workspace"),
		home:    filepath.Join(cfg.RunDir, "home"),
		done:    make(chan error, 1),
	}
	settings, err := s.settings()
	if err != nil {
		return nil, err
	}
	if err := u.WriteFile(filepath.Join(s.baseDir, "userdata", "settings.json"), settings); err != nil {
		return nil, err
	}
	if err := u.WriteFile(filepath.Join(s.work, "README.md"), []byte("Workspace of the lab's fake agent.\n")); err != nil {
		return nil, err
	}
	if err := u.MkdirAll(s.home); err != nil {
		return nil, err
	}
	log, err := u.Create(filepath.Join(cfg.RunDir, "server.log"))
	if err != nil {
		return nil, err
	}
	cmd := u.Command(ctx, cfg.NS, s.env(), s.work, cfg.Node, s.bin(),
		"serve", "--base-dir", s.baseDir, "--port", strconv.Itoa(Port), "--host", "127.0.0.1")
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return nil, errors.Join(err, log.Close())
	}
	cmd.Stderr = log
	if err := cmd.Start(); err != nil {
		return nil, errors.Join(err, log.Close())
	}
	ready := make(chan struct{})
	go func() {
		lines := bufio.NewScanner(stdout)
		signalled := false
		for lines.Scan() {
			if _, err := fmt.Fprintln(log, lines.Text()); err != nil {
				fmt.Fprintf(os.Stderr, "lab: server log: %v\n", err)
			}
			if !signalled && strings.Contains(lines.Text(), "T3 Code server is ready.") {
				signalled = true
				close(ready)
			}
		}
		// Wait closes the pipe, so it runs after the last line is read.
		s.done <- errors.Join(cmd.Wait(), log.Close())
	}()
	select {
	case <-ready:
		return s, nil
	case err := <-s.done:
		return nil, fmt.Errorf("t3 server exited before it was ready (see %s): %w", filepath.Join(cfg.RunDir, "server.log"), err)
	case <-time.After(3 * time.Minute):
		return nil, errors.New("t3 server not ready after 3 minutes")
	}
}

// Exited receives the server's exit error once, when it exits.
func (s *Server) Exited() <-chan error { return s.done }

func (s *Server) bin() string { return filepath.Join(s.cfg.T3Code, "apps", "server", "src", "bin.ts") }

// BaseDir is the server's --base-dir.
func (s *Server) BaseDir() string { return s.baseDir }

// CLI is the command that runs the server's t3 CLI.
func (s *Server) CLI() []string { return []string{s.cfg.Node, s.bin()} }

// env keeps the provider CLIs of this host out of reach: no PATH entry for
// them and a home dir of the server's own. PATH has node's directory for the
// server's own child processes.
func (s *Server) env() []string {
	return []string{
		"HOME=" + s.home,
		"PATH=" + filepath.Dir(s.cfg.Node) + ":/usr/bin:/bin",
		"LANG=C.UTF-8",
		"T3CODE_NO_BROWSER=true",
	}
}

// settings turns off the providers that are on by default, so no prompt can
// reach a real model, and adds the fake agent as provider "lab".
func (s *Server) settings() ([]byte, error) {
	return json.MarshalIndent(map[string]any{
		"defaultRuntimeMode": "approval-required",
		"providerInstances": map[string]any{
			"codex":       map[string]any{"driver": "codex", "enabled": false},
			"claudeAgent": map[string]any{"driver": "claudeAgent", "enabled": false},
			"lab": map[string]any{
				"driver":      "acpRegistry",
				"displayName": "Lab agent",
				"config": map[string]any{
					"source":      "local",
					"commandPath": s.cfg.LabExe,
					"commandArgs": []string{
						"fakeagent",
						"--log", AgentLog(s.cfg.RunDir),
						"--rate", strconv.Itoa(s.cfg.CharsPerSecond),
						"--soak-parts", strconv.Itoa(s.cfg.SoakParts),
					},
				},
			},
		},
	}, "", "  ")
}

// cli runs a t3 subcommand. Callers pass --base-dir before the positional
// arguments.
func (s *Server) cli(ctx context.Context, args ...string) (string, error) {
	args = append([]string{s.bin()}, args...)
	var stdout, stderr strings.Builder
	cmd := s.cfg.User.Command(ctx, s.cfg.NS, s.env(), s.work, s.cfg.Node, args...)
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		return "", fmt.Errorf("t3 %s: %w: %s", strings.Join(args[1:], " "), err, lastLines(stderr.String(), 5))
	}
	return stdout.String(), nil
}

// Setup creates the project and the lab thread, which gets one short warm-up
// turn, and mints a pairing URL for the phone at origin.
func (s *Server) Setup(ctx context.Context, origin string) (threadID, pairURL string, err error) {
	if _, err := s.cli(ctx, "project", "add", "--base-dir", s.baseDir, s.work); err != nil {
		return "", "", err
	}
	out, err := s.cli(ctx, "thread", "start", "--base-dir", s.baseDir, "--provider", "lab", "--model", agent.Model,
		"--title", "Lab thread", "--json", "--wait", s.work, agent.WarmupPrompt)
	if err != nil {
		return "", "", err
	}
	var started struct {
		ThreadID string `json:"threadId"`
		Status   string `json:"status"`
	}
	if err := json.Unmarshal([]byte(lastLines(out, 1)), &started); err != nil {
		return "", "", fmt.Errorf("parse thread start output %q: %w", out, err)
	}
	if started.Status != "completed" {
		return "", "", fmt.Errorf("warm-up turn ended %q", started.Status)
	}
	out, err = s.cli(ctx, "auth", "pairing", "create", "--base-dir", s.baseDir, "--base-url", origin, "--ttl", "1h", "--label", "lab-phone")
	if err != nil {
		return "", "", err
	}
	for _, line := range strings.Split(out, "\n") {
		if url, ok := strings.CutPrefix(line, "Pair URL: "); ok {
			return started.ThreadID, strings.TrimSpace(url), nil
		}
	}
	return "", "", fmt.Errorf("no pairing URL in %q", out)
}

// PairingCode mints a one-time pairing code with `t3 pair`.
func (s *Server) PairingCode(ctx context.Context) (string, error) {
	out, err := s.cli(ctx, "pair", "--base-dir", s.baseDir)
	if err != nil {
		return "", err
	}
	for _, line := range strings.Split(out, "\n") {
		if code, ok := strings.CutPrefix(line, "Token: "); ok {
			return strings.TrimSpace(code), nil
		}
	}
	return "", fmt.Errorf("no code in %q", out)
}

func lastLines(s string, n int) string {
	lines := strings.Split(strings.TrimSpace(s), "\n")
	return strings.Join(lines[max(len(lines)-n, 0):], "\n")
}
