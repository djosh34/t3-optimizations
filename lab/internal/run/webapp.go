package run

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"syscall"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
	"github.com/djosh34/t3-optimizations/lab/internal/profile"
	"github.com/djosh34/t3-optimizations/lab/internal/t3server"
	"github.com/djosh34/t3-optimizations/lab/internal/userproc"
)

// E2EOptions is one end-to-end test of the web app.
type E2EOptions struct {
	Seed int64
	// Outage is how long the link goes down in the outage step.
	Outage time.Duration
	// Deadline bounds the whole test.
	Deadline time.Duration
	OutDir   string
	Name     string
	LabDir   string
	LabExe   string
	Node     string
}

// e2eSteps is the number of steps in driver/webapp-e2e.mjs.
const e2eSteps = 18

// e2eClean is the link between the steps that shape it: weak 4G.
const e2eClean = "A"

// RunWebappE2E runs driver/webapp-e2e.mjs against the sidecar and the stock
// server in the lab, and does what the driver asks: shape the link, cut it,
// block UDP or HTTPS, swap the web app for a newer build, delete a thread
// while the sidecar is down. Returns the headline; the run directory has
// e2e.md, the events and the screenshots. failed counts the steps that
// failed or did not run.
func RunWebappE2E(ctx context.Context, o E2EOptions) (headline string, failed int, err error) {
	sidecarBin, _, webDir, err := sidecarBinaries(o.LabDir, true)
	if err != nil {
		return "", 0, err
	}
	nextDir := filepath.Join(o.LabDir, "bin", "webapp-next")
	if _, err := os.Stat(filepath.Join(nextDir, "sw.js")); err != nil {
		return "", 0, fmt.Errorf("no newer build at %s; run lab/scripts/build-webapp.sh %s next as yourself first", nextDir, nextDir)
	}
	u, err := userproc.Invoking()
	if err != nil {
		return "", 0, err
	}
	if err := netlab.Cleanup(ctx); err != nil {
		return "", 0, fmt.Errorf("cleanup of old labs: %w", err)
	}
	lab, err := netlab.Claim()
	if err != nil {
		return "", 0, err
	}
	// The reaper deletes the lab if this process dies; see Run.
	reaper := exec.Command(o.LabExe, "reap", "--id", lab.ID, "--pid", strconv.Itoa(os.Getpid()))
	reaper.SysProcAttr = &syscall.SysProcAttr{Setsid: true}
	if err := reaper.Start(); err != nil {
		return "", 0, errors.Join(err, lab.Delete(ctx))
	}
	go func() {
		if err := reaper.Wait(); err != nil {
			fmt.Fprintf(os.Stderr, "lab: reaper: %v\n", err)
		}
	}()
	defer func() {
		cleanupCtx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
		defer cancel()
		if err := lab.Delete(cleanupCtx); err != nil {
			fmt.Fprintf(os.Stderr, "lab: cleanup of lab %s: %v\n", lab.ID, err)
		}
	}()
	name := o.Name
	if name == "" {
		name = time.Now().UTC().Format("20060102-150405") + "-webapp-e2e"
	}
	runDir := filepath.Join(o.OutDir, name)
	phoneDir := filepath.Join(runDir, "phone")
	if err := u.MkdirAll(phoneDir); err != nil {
		return "", 0, err
	}
	fmt.Fprintf(os.Stderr, "lab: e2e %s in %s\n", lab.ID, runDir)
	p, err := profile.Get(e2eClean)
	if err != nil {
		return "", 0, err
	}
	s := &scenario{
		o: Options{Profile: p, Seed: o.Seed, Client: ClientWebapp, Kind: "warm", LabDir: o.LabDir, LabExe: o.LabExe, Node: o.Node},
		u: u, lab: lab, runDir: runDir, deadline: o.Deadline,
	}
	e := &e2e{scenario: s, sidecarBin: sidecarBin, webDir: filepath.Join(runDir, "web"), nextDir: nextDir}
	if err := u.CopyDir(webDir, e.webDir); err != nil {
		return "", 0, err
	}
	results, err := e.run(ctx, filepath.Dir(o.LabDir), phoneDir, o)
	if err != nil {
		return "", 0, err
	}
	report := e2eReport(results, buildCommit(), o, countLines(filepath.Join(runDir, "sidecar", "link-reports.jsonl")))
	if err := u.WriteFile(filepath.Join(runDir, "e2e.md"), []byte(report)); err != nil {
		return "", 0, err
	}
	passed := 0
	for _, r := range results {
		if strings.HasPrefix(r.result, "ok") {
			passed++
		}
	}
	failed = e2eSteps - passed
	return fmt.Sprintf("webapp e2e: %d of %d steps passed (%s)", passed, e2eSteps, filepath.Join(runDir, "e2e.md")), failed, nil
}

type e2e struct {
	*scenario
	sidecarBin, webDir, nextDir string
	server                      *t3server.Server
	sidecarStop                 context.CancelFunc
	sidecarExited               <-chan error
	sidecarStarts               int
}

type stepResult struct {
	name, result string
}

func (e *e2e) startSidecar(ctx context.Context) error {
	sctx, cancel := context.WithCancel(ctx)
	// Each start logs on its own, so a restart keeps the log before it.
	e.sidecarStarts++
	logName := "sidecar.log"
	if e.sidecarStarts > 1 {
		logName = fmt.Sprintf("sidecar-%d.log", e.sidecarStarts)
	}
	exited, err := e.startProcess(sctx, e.lab.ServerNS, logName, e.sidecarBin,
		"--t3-url", "http://127.0.0.1:"+strconv.Itoa(t3server.Port),
		"--t3-base-dir", e.server.BaseDir(),
		"--t3-command", strings.Join(e.server.CLI(), " "),
		"--data-dir", filepath.Join(e.runDir, "sidecar"),
		"--http", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarHTTPPort),
		"--udp", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarUDPPort),
		"--web-dir", e.webDir)
	if err != nil {
		cancel()
		return err
	}
	e.sidecarStop, e.sidecarExited = cancel, exited
	return nil
}

func (e *e2e) stopSidecar() {
	e.sidecarStop()
	if err := <-e.sidecarExited; err != nil {
		fmt.Fprintf(os.Stderr, "lab: sidecar stopped: %v\n", err)
	}
}

func (e *e2e) run(parent context.Context, repo, phoneDir string, o E2EOptions) ([]stepResult, error) {
	ctx, fail := context.WithCancelCause(parent)
	defer fail(nil)
	procs, err := e.startLink(ctx, fail, phoneDir)
	if err != nil {
		return nil, err
	}
	defer procs.stop()
	server, err := t3server.Start(procs.ctx, t3server.Config{
		User: e.u, NS: e.lab.ServerNS, LabID: e.lab.ID, T3Code: filepath.Join(repo, "t3code"), RunDir: e.runDir,
		LabExe: o.LabExe, Node: o.Node, CharsPerSecond: CharsPerSecond,
	})
	if err != nil {
		return nil, err
	}
	e.server = server
	procs.watch("t3 server", server.Exited())
	front := "https://" + netlab.ServerAddr + ":" + frontPort
	setupCtx, cancelSetup := context.WithTimeout(ctx, setupTimeout)
	defer cancelSetup()
	threadID, _, err := server.Setup(setupCtx, front)
	if err != nil {
		return nil, err
	}
	code, err := server.PairingCode(setupCtx)
	if err != nil {
		return nil, err
	}
	if err := e.startSidecar(procs.ctx); err != nil {
		return nil, err
	}
	defer e.stopSidecar()
	exited, err := e.startLabProcess(procs.ctx, e.lab.ServerNS, "front.log", "front",
		"--listen", ":"+frontPort, "--upstream", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarHTTPPort))
	if err != nil {
		return nil, err
	}
	procs.watch("front", exited)
	if err := e.shape(ctx, e2eClean); err != nil {
		return nil, err
	}

	// The reply's first sentence, and the first after the approval.
	steps := agent.Script(agent.MeasuredPrompt, 0)
	firstSentence := func(text string) string { return strings.SplitAfter(text, ".")[0] }
	driver, err := phone.StartDriver(ctx, e.lab, e.u, "webapp-e2e.mjs", phone.E2EConfig{
		Origin: front, Code: code, ThreadID: threadID, Prompt: agent.MeasuredPrompt,
		ReplyStart: firstSentence(steps[0].Text), ReplyEnd: firstSentence(steps[4].Text),
		ReplyLast:     replyLast(steps),
		Image:         base64.StdEncoding.EncodeToString(agent.Picture()),
		OutageSeconds: int(o.Outage.Seconds()),
	}, phoneDir, filepath.Join(e.runDir, "driver.log"))
	if err != nil {
		return nil, err
	}
	events, err := e.u.Create(filepath.Join(e.runDir, "phone.jsonl"))
	if err != nil {
		return nil, err
	}
	defer func() {
		if err := events.Close(); err != nil {
			fmt.Fprintf(os.Stderr, "lab: close phone events: %v\n", err)
		}
	}()
	deadline := time.After(o.Deadline)
	var results []stepResult
	for {
		select {
		case ev, ok := <-driver.Events:
			if !ok {
				// The driver exits 1 when a step failed; the steps say which.
				if err := driver.Wait(); err != nil {
					fmt.Fprintf(os.Stderr, "lab: driver: %v\n", err)
				}
				return results, nil
			}
			line, err := json.Marshal(ev)
			if err != nil {
				return nil, err
			}
			if _, err := events.Write(append(line, '\n')); err != nil {
				return nil, err
			}
			switch ev.Ev {
			case "step":
				fmt.Fprintf(os.Stderr, "lab: step %s: %s\n", ev.Name, ev.Text)
				results = append(results, stepResult{ev.Name, ev.Text})
			case "ask":
				answer := "ok"
				value, err := e.answer(ctx, ev.Text)
				if err != nil {
					answer = "error " + strings.ReplaceAll(err.Error(), "\n", " ")
				} else if value != "" {
					answer += " " + value
				}
				fmt.Fprintf(os.Stderr, "lab: %s: %s\n", ev.Text, answer)
				if err := driver.Send(answer); err != nil {
					return nil, err
				}
			}
		case <-deadline:
			return append(results, stepResult{"deadline", fmt.Sprintf("failed: not done in %s", o.Deadline)}), nil
		case <-ctx.Done():
			return nil, context.Cause(ctx)
		}
	}
}

// replyLast is the text of the reply's last line between its last quotes:
// the string the code block ends with, one token when it is highlighted.
func replyLast(steps []agent.Step) string {
	text := strings.TrimSpace(steps[len(steps)-1].Text)
	line := text[strings.LastIndex(text, "\n")+1:]
	if parts := strings.Split(line, `"`); len(parts) >= 3 {
		return parts[len(parts)-2]
	}
	return line
}

func (e *e2e) shape(ctx context.Context, name string) error {
	p, err := profile.Get(name)
	if err != nil {
		return err
	}
	return e.lab.Shape(ctx, p.Down, p.Up, e.o.Seed)
}

// answer does what the driver asked.
func (e *e2e) answer(ctx context.Context, ask string) (string, error) {
	verb, arg, _ := strings.Cut(ask, " ")
	switch verb {
	case "shape":
		if arg == "clean" {
			arg = e2eClean
		}
		return "", e.shape(ctx, arg)
	case "outage":
		return "", e.lab.SetOutage(ctx, arg == "on")
	case "block":
		what, state, _ := strings.Cut(arg, " ")
		return "", e.lab.Block(ctx, what, state == "on")
	case "swap":
		// The newer build's files over the served ones; its hashed assets
		// sit next to the old ones, as on a real server mid-update.
		return "", e.u.CopyDir(e.nextDir, e.webDir)
	case "mint-code":
		return e.server.PairingCode(ctx)
	case "delete-offline":
		return "", e.deleteOffline(ctx, arg)
	}
	return "", fmt.Errorf("unknown ask %q", ask)
}

// deleteOffline stops the sidecar, deletes a thread through the stock
// server's own RPC, and starts the sidecar again.
func (e *e2e) deleteOffline(ctx context.Context, thread string) error {
	e.stopSidecar()
	code, err := e.server.PairingCode(ctx)
	if err != nil {
		return err
	}
	command, err := json.Marshal(map[string]string{
		"type": "thread.delete", "commandId": fmt.Sprintf("lab-delete-%d", time.Now().UnixNano()), "threadId": thread,
	})
	if err != nil {
		return err
	}
	cmd := e.u.Command(ctx, e.lab.ServerNS, []string{"PATH=/usr/bin:/bin"}, e.runDir, e.o.Node,
		filepath.Join(e.o.LabDir, "scripts", "t3-command.mjs"), "http://127.0.0.1:"+strconv.Itoa(t3server.Port), code, string(command))
	out, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("delete %s: %w: %s", thread, err, strings.TrimSpace(string(out)))
	}
	return e.startSidecar(ctx)
}

// countLines counts the lines of a file; 0 when there is none.
func countLines(path string) int {
	data, err := os.ReadFile(path)
	if err != nil {
		return 0
	}
	return strings.Count(string(data), "\n")
}

func e2eReport(results []stepResult, commit string, o E2EOptions, linkReports int) string {
	var b strings.Builder
	fmt.Fprintf(&b, "# Web app end to end\n\nCommit %s, seed %d, outage %s. ", commit, o.Seed, o.Outage)
	b.WriteString("WebKit (Playwright 1.64.1) at the iPhone 17 Pro's size in the lab's phone container; ")
	b.WriteString("the sidecar and the stock t3 server in the server namespace, the lab front for HTTPS. ")
	fmt.Fprintf(&b, "Between steps that shape it, the link is profile %s.\n\n", e2eClean)
	b.WriteString("| Step | Result |\n|---|---|\n")
	for _, r := range results {
		fmt.Fprintf(&b, "| %s | %s |\n", r.name, strings.ReplaceAll(r.result, "|", "\\|"))
	}
	fmt.Fprintf(&b, "\nThe app posted %d link reports during the test; the sidecar logged them in `sidecar/link-reports.jsonl`.\n", linkReports)
	b.WriteString("\nScreenshots are in `phone/`: each name once as the Safari tab (`-safari`) and once as the home-screen app (`-home-screen`).\n")
	return b.String()
}
