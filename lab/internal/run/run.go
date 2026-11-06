// Package run runs one lab scenario from setup to report.
package run

import (
	"context"
	"crypto/rand"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
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
	// Client is websocket or websocket-nodeflate.
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

const frontPort = "8443"
const tunnelPort = "7000"

// Run runs the scenario and writes its report.
func Run(ctx context.Context, o Options) (report.Report, error) {
	if o.Client != "websocket" && o.Client != "websocket-nodeflate" {
		return report.Report{}, fmt.Errorf("unknown client %q: use websocket or websocket-nodeflate", o.Client)
	}
	if o.Kind != "warm" && o.Kind != "cold" {
		return report.Report{}, fmt.Errorf("unknown kind %q: use warm or cold", o.Kind)
	}
	deadline := o.Deadline
	if deadline == 0 {
		deadline = o.Profile.Deadline()
	}
	u, err := userproc.Invoking()
	if err != nil {
		return report.Report{}, err
	}
	repo := filepath.Dir(o.LabDir)
	staticDir := filepath.Join(u.Home, ".cache", "t3lab", "baseline-client-"+BaselineCommit[:12], "dist")
	if _, err := os.Stat(filepath.Join(staticDir, "index.html")); err != nil {
		return report.Report{}, fmt.Errorf("no baseline client build at %s; run lab/scripts/build-baseline-client.sh as yourself first", staticDir)
	}
	commit := describeCommit(ctx, u, repo)

	if err := netlab.Cleanup(ctx); err != nil {
		return report.Report{}, fmt.Errorf("cleanup of old labs: %w", err)
	}
	idBytes := make([]byte, 2)
	if _, err := rand.Read(idBytes); err != nil {
		return report.Report{}, err
	}
	lab := netlab.New(hex.EncodeToString(idBytes))
	if err := lab.Claim(); err != nil {
		return report.Report{}, err
	}
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
		return report.Report{}, err
	}
	fmt.Fprintf(os.Stderr, "lab: run %s in %s\n", lab.ID, runDir)

	s := &scenario{o: o, u: u, lab: lab, runDir: runDir, deadline: deadline}
	in, err := s.execute(ctx, repo, staticDir, phoneDir)
	if err != nil {
		return report.Report{}, err
	}
	in.Commit = commit
	r := report.Build(in)
	if err := r.Write(runDir, u.WriteFile); err != nil {
		return report.Report{}, err
	}
	return r, nil
}

type scenario struct {
	o        Options
	u        userproc.User
	lab      *netlab.Lab
	runDir   string
	deadline time.Duration
}

func (s *scenario) execute(ctx context.Context, repo, staticDir, phoneDir string) (report.Input, error) {
	o, u, lab := s.o, s.u, s.lab
	if err := phone.EnsureImage(ctx, filepath.Join(o.LabDir, "driver")); err != nil {
		return report.Input{}, err
	}
	if err := lab.CreateServer(ctx); err != nil {
		return report.Input{}, err
	}
	pid, err := phone.StartContainer(ctx, lab, u, filepath.Join(o.LabDir, "driver"), phoneDir)
	if err != nil {
		return report.Input{}, err
	}
	if err := lab.AttachPhone(ctx, pid); err != nil {
		return report.Input{}, err
	}

	// Server-side processes stop when procs is cancelled, before cleanup.
	procs, stopProcs := context.WithCancel(ctx)
	var waits []func() error
	defer func() {
		stopProcs()
		for _, wait := range waits {
			if err := wait(); err != nil {
				fmt.Fprintf(os.Stderr, "lab: process: %v\n", err)
			}
		}
	}()

	origin := "https://" + netlab.ServerAddr + ":" + frontPort
	if o.Profile.TCPPath {
		if err := lab.CreateTunnel(ctx, u.UID); err != nil {
			return report.Input{}, err
		}
		for _, side := range []struct{ ns, flag, addr, log string }{
			{lab.ServerNS, "--listen", netlab.ServerAddr + ":" + tunnelPort, "tunnel-server.log"},
			{lab.PhoneNS, "--connect", netlab.ServerAddr + ":" + tunnelPort, "tunnel-phone.log"},
		} {
			wait, err := s.startLabProcess(procs, side.ns, side.log, "tunnel", "--dev", lab.TunnelIf(), side.flag, side.addr)
			if err != nil {
				return report.Input{}, err
			}
			waits = append(waits, wait)
		}
		origin = "https://" + netlab.TunnelServerAddr + ":" + frontPort
	}

	soak := 0
	if o.Profile.Soak {
		soak = soakParts
	}
	server, err := t3server.Start(procs, t3server.Config{
		User: u, NS: lab.ServerNS, T3Code: filepath.Join(repo, "t3code"), RunDir: s.runDir,
		LabExe: o.LabExe, StaticDir: staticDir, Node: o.Node, CharsPerSecond: CharsPerSecond, SoakParts: soak,
	})
	if err != nil {
		return report.Input{}, err
	}
	waits = append(waits, server.Wait)
	frontArgs := []string{"front", "--listen", ":" + frontPort, "--upstream", fmt.Sprintf("127.0.0.1:%d", t3server.Port)}
	if o.Client == "websocket-nodeflate" {
		frontArgs = append(frontArgs, "--no-deflate")
	}
	wait, err := s.startLabProcess(procs, lab.ServerNS, "front.log", frontArgs...)
	if err != nil {
		return report.Input{}, err
	}
	waits = append(waits, wait)

	threadID, pairURL, err := server.Setup(ctx, origin)
	if err != nil {
		return report.Input{}, err
	}
	steps := agent.Script(agent.MeasuredPrompt, soak)
	reply := agent.Normalize(agent.ReplyText(steps))
	driver, err := phone.StartDriver(ctx, lab, u, phone.Config{
		Origin: origin, PairURL: pairURL, ThreadID: threadID,
		Prompt: agent.MeasuredPrompt, ImagePrompt: agent.ImagePrompt,
		Image: base64.StdEncoding.EncodeToString(agent.Picture()),
		Reply: reply, Kind: o.Kind,
	}, filepath.Join(s.runDir, "driver.log"))
	if err != nil {
		return report.Input{}, err
	}
	events, err := u.Create(filepath.Join(s.runDir, "phone.jsonl"))
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
	}, nil
}

// startLabProcess runs a lab subcommand as the user in a namespace.
func (s *scenario) startLabProcess(ctx context.Context, ns, logName string, args ...string) (func() error, error) {
	log, err := s.u.Create(filepath.Join(s.runDir, logName))
	if err != nil {
		return nil, err
	}
	cmd := s.u.Command(ctx, ns, []string{"PATH=/usr/bin:/bin"}, s.runDir, s.o.LabExe, args...)
	cmd.Stdout = log
	cmd.Stderr = log
	if err := cmd.Start(); err != nil {
		return nil, errors.Join(err, log.Close())
	}
	return func() error {
		err := cmd.Wait()
		if ctx.Err() != nil {
			// Stopped by the lab.
			err = nil
		}
		return errors.Join(err, log.Close())
	}, nil
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
			return ctx.Err()
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

// describeCommit names the commit the lab runs from, as the user, since git
// refuses a repo that root does not own.
func describeCommit(ctx context.Context, u userproc.User, repo string) string {
	out, err := u.Command(ctx, "", []string{"PATH=/usr/bin:/bin", "HOME=" + u.Home}, repo,
		"git", "describe", "--always", "--dirty", "--abbrev=12").Output()
	if err != nil {
		return "unknown"
	}
	return strings.TrimSpace(string(out))
}
