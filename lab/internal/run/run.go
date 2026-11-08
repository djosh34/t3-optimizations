// Package run runs one lab scenario from setup to report.
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
	"runtime/debug"
	"strconv"
	"strings"
	"syscall"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
	"github.com/djosh34/t3-optimizations/lab/internal/pipereport"
	"github.com/djosh34/t3-optimizations/lab/internal/profile"
	"github.com/djosh34/t3-optimizations/lab/internal/report"
	"github.com/djosh34/t3-optimizations/lab/internal/t3server"
	"github.com/djosh34/t3-optimizations/lab/internal/userproc"
)

// BaselineCommit is the commit whose t3code/ the baseline web client is
// built from; lab/scripts/build-baseline-client.sh names the same one.
const BaselineCommit = "22099f5fdb147f0efee0704fb21b4e19946d736f"

// CharsPerSecond is the fake agent's reply rate in every run.
const CharsPerSecond = 50

// soakParts makes the soak reply about 55 minutes long at CharsPerSecond.
const soakParts = 450

// Options select a scenario.
type Options struct {
	Profile profile.Profile
	Seed    int64
	// Client is websocket, websocket-nodeflate or sidecar-native, or a pipe
	// client: pipe, pipe-unpatched or udp.
	Client string
	// Kind is warm or cold.
	Kind string
	// Deadline overrides the profile's deadline when set.
	Deadline time.Duration
	OutDir   string
	// Name is the run's directory under OutDir. By default it is made of
	// the time, profile, client, kind and seed.
	Name   string
	LabDir string
	LabExe string
	// Node runs the t3 server.
	Node string
}

// setupTimeout bounds the warm-up turn and the pairing, before the profile.
const setupTimeout = 10 * time.Minute

const frontPort = "8443"
const tunnelPort = "7000"

// Run runs the scenario, writes its report and returns the report's
// headline.
func Run(ctx context.Context, o Options) (string, error) {
	pipe := pipereport.IsPipeClient(o.Client)
	webapp := o.Client == ClientWebapp
	sidecar := o.Client == ClientSidecarNative || webapp
	if o.Client != "websocket" && o.Client != "websocket-nodeflate" && !pipe && !sidecar {
		return "", fmt.Errorf("unknown client %q: use websocket, websocket-nodeflate, sidecar-native, webapp, pipe, pipe-unpatched or udp", o.Client)
	}
	if o.Client == ClientSidecarNative && o.Profile.TCPPath {
		return "", fmt.Errorf("profile %s carries the link in a TCP tunnel, which the native phone does not use yet; use --client webapp", o.Profile.Name)
	}
	if o.Kind != "warm" && o.Kind != "cold" {
		return "", fmt.Errorf("unknown kind %q: use warm or cold", o.Kind)
	}
	if pipe && o.Kind != "warm" {
		return "", errors.New("pipe runs are warm only")
	}
	deadline := o.Deadline
	if deadline == 0 {
		deadline = o.Profile.Deadline()
		if pipe {
			deadline = PipeDeadline
		}
	}
	u, err := userproc.Invoking()
	if err != nil {
		return "", err
	}
	repo := filepath.Dir(o.LabDir)
	staticDir := filepath.Join(u.Home, ".cache", "t3lab", "baseline-client-"+BaselineCommit[:12], "dist")
	var server, sidecarBin, nativeBin, webDir string
	if pipe {
		if server, err = pipeServer(o.LabDir, o.Client); err != nil {
			return "", err
		}
	} else if sidecar {
		if sidecarBin, nativeBin, webDir, err = sidecarBinaries(o.LabDir, webapp); err != nil {
			return "", err
		}
	} else if _, err := os.Stat(filepath.Join(staticDir, "index.html")); err != nil {
		return "", fmt.Errorf("no baseline client build at %s; run lab/scripts/build-baseline-client.sh as yourself first", staticDir)
	}
	commit := buildCommit()

	if err := netlab.Cleanup(ctx); err != nil {
		return "", fmt.Errorf("cleanup of old labs: %w", err)
	}
	lab, err := netlab.Claim()
	if err != nil {
		return "", err
	}
	// The reaper is its own session, so it outlives this process if it is
	// killed, and then deletes the lab's namespaces and container.
	reaper := exec.Command(o.LabExe, "reap", "--id", lab.ID, "--pid", strconv.Itoa(os.Getpid()))
	reaper.SysProcAttr = &syscall.SysProcAttr{Setsid: true}
	if err := reaper.Start(); err != nil {
		return "", errors.Join(err, lab.Delete(ctx))
	}
	go func() {
		if err := reaper.Wait(); err != nil {
			fmt.Fprintf(os.Stderr, "lab: reaper: %v\n", err)
		}
	}()
	// The run's own context may be cancelled by Ctrl-C; cleanup still runs.
	defer func() {
		cleanupCtx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
		defer cancel()
		if err := lab.Delete(cleanupCtx); err != nil {
			fmt.Fprintf(os.Stderr, "lab: cleanup of lab %s: %v\n", lab.ID, err)
		}
	}()

	name := o.Name
	if name == "" {
		name = fmt.Sprintf("%s-%s-%s-%s-s%d",
			time.Now().UTC().Format("20060102-150405"), o.Profile.Name, o.Client, o.Kind, o.Seed)
	}
	runDir := filepath.Join(o.OutDir, name)
	phoneDir := filepath.Join(runDir, "phone")
	if err := u.MkdirAll(phoneDir); err != nil {
		return "", err
	}
	fmt.Fprintf(os.Stderr, "lab: run %s in %s\n", lab.ID, runDir)

	s := &scenario{o: o, u: u, lab: lab, runDir: runDir, deadline: deadline}
	if pipe {
		in, err := s.executePipe(ctx, server, phoneDir)
		if err != nil {
			return "", err
		}
		in.Commit = commit
		r := pipereport.Build(in)
		return r.Headline(), r.Write(runDir, u.WriteFile)
	}
	var in report.Input
	if sidecar {
		in, err = s.executeSidecar(ctx, repo, phoneDir, sidecarBin, nativeBin, webDir)
	} else {
		in, err = s.execute(ctx, repo, staticDir, phoneDir)
	}
	if err != nil {
		return "", err
	}
	in.Commit = commit
	r := report.Build(in)
	return r.Headline(), r.Write(runDir, u.WriteFile)
}

type scenario struct {
	o        Options
	u        userproc.User
	lab      *netlab.Lab
	runDir   string
	deadline time.Duration
}

func (s *scenario) execute(parent context.Context, repo, staticDir, phoneDir string) (in report.Input, err error) {
	o, u, lab := s.o, s.u, s.lab
	// A lab process that exits before the lab stops it fails the run, and
	// the run's error says why.
	ctx, fail := context.WithCancelCause(parent)
	defer func() {
		if err != nil && ctx.Err() != nil && parent.Err() == nil {
			err = context.Cause(ctx)
		}
		fail(nil)
	}()
	procs, err := s.startLink(ctx, fail, phoneDir)
	if err != nil {
		return report.Input{}, err
	}
	defer procs.stop()

	serverAddr, err := s.startTunnel(ctx, procs)
	if err != nil {
		return report.Input{}, err
	}
	origin := "https://" + serverAddr + ":" + frontPort

	soak := 0
	if o.Profile.Soak {
		soak = soakParts
	}
	server, err := t3server.Start(procs.ctx, t3server.Config{
		User: u, NS: lab.ServerNS, LabID: lab.ID, T3Code: filepath.Join(repo, "t3code"), RunDir: s.runDir,
		LabExe: o.LabExe, StaticDir: staticDir, Node: o.Node, CharsPerSecond: CharsPerSecond, SoakParts: soak,
	})
	if err != nil {
		return report.Input{}, err
	}
	procs.watch("t3 server", server.Exited())
	frontArgs := []string{"front", "--listen", ":" + frontPort, "--upstream", fmt.Sprintf("127.0.0.1:%d", t3server.Port)}
	if o.Client == "websocket-nodeflate" {
		frontArgs = append(frontArgs, "--no-deflate")
	}
	exited, err := s.startLabProcess(procs.ctx, lab.ServerNS, "front.log", frontArgs...)
	if err != nil {
		return report.Input{}, err
	}
	procs.watch("front", exited)

	// Setup runs on the clean link; a hang there is a lab fault, not a result.
	setupCtx, cancelSetup := context.WithTimeout(ctx, setupTimeout)
	defer cancelSetup()
	threadID, pairURL, err := server.Setup(setupCtx, origin)
	if err != nil {
		return report.Input{}, err
	}
	steps := agent.Script(agent.MeasuredPrompt, soak)
	reply := agent.Normalize(agent.ReplyText(steps))
	driver, err := phone.StartDriver(ctx, lab, u, "driver.mjs", phone.Config{
		Origin: origin, PairURL: pairURL, ThreadID: threadID,
		Prompt: agent.MeasuredPrompt, ImagePrompt: agent.ImagePrompt,
		Image: base64.StdEncoding.EncodeToString(agent.Picture()),
		Reply: reply, Kind: o.Kind,
	}, phoneDir, filepath.Join(s.runDir, "driver.log"))
	if err != nil {
		return report.Input{}, err
	}
	return s.drive(ctx, driver, reply, "")
}

// startTunnel sets up the TCP outer path when the profile asks for it, and
// returns the server's address as the phone reaches it: through the tunnel,
// or on the veth pair.
func (s *scenario) startTunnel(ctx context.Context, procs *processes) (string, error) {
	if !s.o.Profile.TCPPath {
		return netlab.ServerAddr, nil
	}
	lab := s.lab
	if err := lab.CreateTunnel(ctx, s.u.UID); err != nil {
		return "", err
	}
	for _, side := range []struct{ ns, flag, addr, log string }{
		{lab.ServerNS, "--listen", netlab.ServerAddr + ":" + tunnelPort, "tunnel-server.log"},
		{lab.PhoneNS, "--connect", netlab.ServerAddr + ":" + tunnelPort, "tunnel-phone.log"},
	} {
		exited, err := s.startLabProcess(procs.ctx, side.ns, side.log, "tunnel", "--dev", lab.TunnelIf(), side.flag, side.addr)
		if err != nil {
			return "", err
		}
		procs.watch(side.log, exited)
	}
	return netlab.TunnelServerAddr, nil
}

// drive runs the measured part of a t3 scenario with a started phone
// driver: waits until it is ready, applies the profile, sends "go", injects
// the faults, and collects events until the reply is complete or the
// deadline passes. path is the report's note on the transport.
func (s *scenario) drive(ctx context.Context, driver *phone.Driver, reply, path string) (report.Input, error) {
	o, lab := s.o, s.lab
	events, err := s.u.Create(filepath.Join(s.runDir, "phone.jsonl"))
	if err != nil {
		return report.Input{}, err
	}
	defer func() {
		if err := events.Close(); err != nil {
			fmt.Fprintf(os.Stderr, "lab: close phone events: %v\n", err)
		}
	}()
	var phoneEvents []phone.Event
	record := func(ev phone.Event) error {
		phoneEvents = append(phoneEvents, ev)
		line, err := json.Marshal(ev)
		if err != nil {
			return err
		}
		_, err = events.Write(append(line, '\n'))
		return err
	}

	if err := s.waitReady(ctx, driver, record); err != nil {
		return report.Input{}, err
	}
	before, err := lab.ReadCounters(ctx)
	if err != nil {
		return report.Input{}, err
	}
	switched := time.Now()
	if err := lab.Shape(ctx, o.Profile.Down, o.Profile.Up, o.Seed); err != nil {
		return report.Input{}, err
	}
	if err := driver.Send("go"); err != nil {
		return report.Input{}, err
	}
	fmt.Fprintf(os.Stderr, "lab: profile %s applied, deadline %s\n", o.Profile.Name, s.deadline)
	// A failed fault injection ends the measurement with its error.
	measureCtx, abort := context.WithCancelCause(ctx)
	faultCtx, stopFaults := context.WithCancel(measureCtx)
	faultsDone := make(chan struct{})
	go func() {
		defer close(faultsDone)
		if err := s.injectFaults(faultCtx, switched); err != nil {
			abort(fmt.Errorf("fault injection: %w", err))
		}
	}()
	end, err := s.measure(measureCtx, driver, record, switched)
	stopFaults()
	<-faultsDone
	abort(nil)
	if err != nil {
		return report.Input{}, err
	}
	after, err := lab.ReadCounters(ctx)
	if err != nil {
		return report.Input{}, err
	}
	agentLog, err := readAgentLog(t3server.AgentLog(s.runDir))
	if err != nil {
		return report.Input{}, err
	}
	return report.Input{
		Profile: o.Profile.Name, Grade: o.Profile.Grade(), About: o.Profile.About,
		Client: o.Client, Kind: o.Kind, Seed: o.Seed, Deadline: s.deadline,
		Switched: switched, End: end, Reply: reply,
		Agent: agentLog, Phone: phoneEvents,
		WireDown: after.Down - before.Down, WireUp: after.Up - before.Up,
		Path: path,
	}, nil
}

// startLabProcess runs a lab subcommand as the user in a namespace. The
// channel receives its exit error once.
func (s *scenario) startLabProcess(ctx context.Context, ns, logName string, args ...string) (<-chan error, error) {
	return s.startProcess(ctx, ns, logName, s.o.LabExe, args...)
}

// startProcess runs exe as the user in a namespace, with its output in
// logName in the run dir. The channel receives its exit error once.
func (s *scenario) startProcess(ctx context.Context, ns, logName, exe string, args ...string) (<-chan error, error) {
	log, err := s.u.Create(filepath.Join(s.runDir, logName))
	if err != nil {
		return nil, err
	}
	cmd := s.u.Command(ctx, ns, []string{"PATH=/usr/bin:/bin"}, s.runDir, exe, args...)
	cmd.Stdout = log
	cmd.Stderr = log
	if err := cmd.Start(); err != nil {
		return nil, errors.Join(err, log.Close())
	}
	exited := make(chan error, 1)
	go func() { exited <- errors.Join(cmd.Wait(), log.Close()) }()
	return exited, nil
}

// processes are a run's server-side processes. They stop when stop is
// called, before the lab's cleanup; one that exits before then fails the
// run.
type processes struct {
	ctx    context.Context
	cancel context.CancelFunc
	fail   context.CancelCauseFunc
	waits  []func() error
}

// startLink creates the server namespace and the phone container joined by
// the link, and returns the run's processes, none started yet. fail fails
// the run.
func (s *scenario) startLink(ctx context.Context, fail context.CancelCauseFunc, phoneDir string) (*processes, error) {
	driverDir := filepath.Join(s.o.LabDir, "driver")
	if err := phone.EnsureImage(ctx, driverDir); err != nil {
		return nil, err
	}
	if err := s.lab.CreateServer(ctx); err != nil {
		return nil, err
	}
	pid, err := phone.StartContainer(ctx, s.lab, s.u, driverDir, phoneDir)
	if err != nil {
		return nil, err
	}
	if err := s.lab.AttachPhone(ctx, pid); err != nil {
		return nil, err
	}
	procs, cancel := context.WithCancel(ctx)
	return &processes{ctx: procs, cancel: cancel, fail: fail}, nil
}

// watch fails the run when the process exits before stop.
func (p *processes) watch(name string, exited <-chan error) {
	p.waits = append(p.waits, watch(p.ctx, p.fail, name, exited))
}

// stop stops the processes and waits for them.
func (p *processes) stop() {
	p.cancel()
	for _, wait := range p.waits {
		if err := wait(); err != nil {
			fmt.Fprintf(os.Stderr, "lab: process: %v\n", err)
		}
	}
}

// watch fails the run when a process exits before procs ends, and returns
// a function that waits for the exit.
func watch(procs context.Context, fail context.CancelCauseFunc, name string, exited <-chan error) func() error {
	result := make(chan error, 1)
	go func() {
		err := <-exited
		if procs.Err() != nil {
			// Stopped by the lab.
			result <- nil
			return
		}
		err = fmt.Errorf("%s exited during the run: %v", name, err)
		fail(err)
		result <- err
	}()
	return func() error { return <-result }
}

// waitReady waits until the phone is paired, has the thread open and has sent
// the setup turn, all on the clean link.
func (s *scenario) waitReady(ctx context.Context, driver *phone.Driver, record func(phone.Event) error) error {
	timeout := time.After(5 * time.Minute)
	for {
		select {
		case ev, ok := <-driver.Events:
			if !ok {
				return fmt.Errorf("driver exited during setup: %v (see %s)", driver.Wait(), filepath.Join(s.runDir, "driver.log"))
			}
			if err := record(ev); err != nil {
				return err
			}
			if ev.Ev == "ready" {
				return nil
			}
		case <-timeout:
			return fmt.Errorf("phone not ready after 5 minutes on a clean link (see %s)", filepath.Join(s.runDir, "driver.log"))
		case <-ctx.Done():
			return context.Cause(ctx)
		}
	}
}

// measure collects the driver's events until the reply is complete or the
// deadline passes, and returns when the measured part ended.
func (s *scenario) measure(ctx context.Context, driver *phone.Driver, record func(phone.Event) error,
	switched time.Time) (time.Time, error) {
	deadline := time.NewTimer(time.Until(switched.Add(s.deadline)))
	defer deadline.Stop()
	end := time.Time{}
	stop := func() error {
		if end.IsZero() {
			end = time.Now()
		}
		return driver.Send("stop")
	}
	drain := time.NewTimer(time.Hour)
	drain.Stop()
	for {
		select {
		case ev, ok := <-driver.Events:
			if !ok {
				if err := driver.Wait(); err != nil {
					return time.Time{}, fmt.Errorf("driver: %w (see %s)", err, filepath.Join(s.runDir, "driver.log"))
				}
				if end.IsZero() {
					end = time.Now()
				}
				return end, nil
			}
			if err := record(ev); err != nil {
				return time.Time{}, err
			}
			if ev.Ev == "complete" && end.IsZero() {
				end = time.UnixMilli(ev.T)
			}
		case <-deadline.C:
			fmt.Fprintf(os.Stderr, "lab: deadline reached\n")
			if err := stop(); err != nil {
				return time.Time{}, err
			}
			drain.Reset(2 * time.Minute)
		case <-drain.C:
			return time.Time{}, errors.New("driver did not exit within 2 minutes of the stop")
		case <-ctx.Done():
			return time.Time{}, context.Cause(ctx)
		}
	}
}

// injectFaults applies the profile's outages and address change on time.
func (s *scenario) injectFaults(ctx context.Context, switched time.Time) error {
	p := s.o.Profile
	down, moved := false, false
	ticker := time.NewTicker(100 * time.Millisecond)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			if down {
				cleanupCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
				defer cancel()
				return s.lab.SetOutage(cleanupCtx, false)
			}
			return nil
		case <-ticker.C:
		}
		offset := time.Since(switched)
		if want := p.Outages.OutageAt(offset); want != down {
			if err := s.lab.SetOutage(ctx, want); err != nil {
				return err
			}
			down = want
		}
		if p.AddrChange > 0 && !moved && offset >= p.AddrChange {
			if err := s.lab.ChangePhoneAddr(ctx); err != nil {
				return err
			}
			moved = true
		}
	}
}

func readAgentLog(path string) ([]agent.LogEvent, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	var out []agent.LogEvent
	for _, line := range strings.Split(strings.TrimSpace(string(data)), "\n") {
		if line == "" {
			continue
		}
		var ev agent.LogEvent
		if err := json.Unmarshal([]byte(line), &ev); err != nil {
			return nil, fmt.Errorf("%s: %w", path, err)
		}
		out = append(out, ev)
	}
	return out, nil
}

// buildCommit names the commit this lab binary was built from, with
// "-dirty" when the checkout had uncommitted changes.
func buildCommit() string {
	info, ok := debug.ReadBuildInfo()
	if !ok {
		return "unknown"
	}
	revision, dirty := "unknown", ""
	for _, setting := range info.Settings {
		switch {
		case setting.Key == "vcs.revision":
			revision = setting.Value[:min(12, len(setting.Value))]
		case setting.Key == "vcs.modified" && setting.Value == "true":
			dirty = "-dirty"
		}
	}
	return revision + dirty
}
