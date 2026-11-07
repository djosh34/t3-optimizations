package run

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
	"github.com/djosh34/t3-optimizations/lab/internal/pipereport"
)

// PipeDeadline is how long a pipe run measures after the switch, unless
// --deadline says otherwise.
const PipeDeadline = 2 * time.Minute

// The pipe server's ports in the server namespace. The front serves the page
// and the offer on frontPort, like the t3 runs.
const (
	pipeUDPPort  = 12444
	pipeHTTPPort = "8090"
)

// pipeServer returns the pipe-lab binary for a pipe client, built by
// lab/scripts/build-pipe-lab.sh.
func pipeServer(labDir, client string) (string, error) {
	name := "pipe-lab"
	if client == pipereport.ClientPipeUnpatched {
		name = "pipe-lab-unpatched"
	}
	path := filepath.Join(labDir, "bin", name)
	if _, err := os.Stat(path); err != nil {
		return "", fmt.Errorf("no %s; run lab/scripts/build-pipe-lab.sh as yourself first", path)
	}
	return path, nil
}

// executePipe runs a pipe scenario: ticks both ways between the pipe server
// and the phone, first on the clean link, then under the profile until the
// deadline.
func (s *scenario) executePipe(parent context.Context, server, phoneDir string) (in pipereport.Input, err error) {
	o, lab := s.o, s.lab
	ctx, fail := context.WithCancelCause(parent)
	defer func() {
		if err != nil && ctx.Err() != nil && parent.Err() == nil {
			err = context.Cause(ctx)
		}
		fail(nil)
	}()
	procs, err := s.startLink(ctx, fail, phoneDir)
	if err != nil {
		return in, err
	}
	defer procs.stop()
	start := func(log, exe string, args ...string) error {
		exited, err := s.startProcess(procs.ctx, lab.ServerNS, log, exe, args...)
		if err != nil {
			return err
		}
		procs.watch(log, exited)
		return nil
	}

	serverLog := filepath.Join(s.runDir, "server.jsonl")
	udp := netlab.ServerAddr + ":" + strconv.Itoa(pipeUDPPort)
	cfg := phone.PipeConfig{Mode: "webrtc", Origin: "https://" + netlab.ServerAddr + ":" + frontPort}
	if o.Client == pipereport.ClientUDP {
		cfg = phone.PipeConfig{Mode: "udp", UDPHost: netlab.ServerAddr, UDPPort: pipeUDPPort}
		if err := start("pipe-lab.log", server, "udp", "--udp", udp, "--log", serverLog); err != nil {
			return in, err
		}
	} else {
		http := "127.0.0.1:" + pipeHTTPPort
		if err := start("pipe-lab.log", server, "webrtc", "--udp", udp, "--http", http, "--log", serverLog); err != nil {
			return in, err
		}
		if err := start("front.log", o.LabExe, "front", "--listen", ":"+frontPort, "--upstream", http); err != nil {
			return in, err
		}
	}

	driver, err := phone.StartDriver(ctx, lab, s.u, "pipe.mjs", cfg, phoneDir, filepath.Join(s.runDir, "driver.log"))
	if err != nil {
		return in, err
	}
	events, err := s.u.Create(filepath.Join(s.runDir, "phone.jsonl"))
	if err != nil {
		return in, err
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
		return in, err
	}
	// A few seconds of ticks on the clean link first, so the report shows
	// the switch, not the connection setup.
	if err := collect(ctx, driver, record, time.Now().Add(3*time.Second)); err != nil {
		return in, err
	}

	before, err := lab.ReadCounters(ctx)
	if err != nil {
		return in, err
	}
	switched := time.Now()
	if err := lab.Shape(ctx, o.Profile.Down, o.Profile.Up, o.Seed); err != nil {
		return in, err
	}
	if err := driver.Send("go"); err != nil {
		return in, err
	}
	fmt.Fprintf(os.Stderr, "lab: profile %s applied, measuring for %s\n", o.Profile.Name, s.deadline)
	measureCtx, abort := context.WithCancelCause(ctx)
	faultCtx, stopFaults := context.WithCancel(measureCtx)
	faultsDone := make(chan struct{})
	go func() {
		defer close(faultsDone)
		if err := s.injectFaults(faultCtx, switched); err != nil {
			abort(fmt.Errorf("fault injection: %w", err))
		}
	}()
	end := switched.Add(s.deadline)
	err = collect(measureCtx, driver, record, end)
	stopFaults()
	<-faultsDone
	abort(nil)
	if err != nil {
		return in, err
	}
	if err := driver.Send("stop"); err != nil {
		return in, err
	}
	if err := drainDriver(ctx, driver, record); err != nil {
		return in, err
	}
	after, err := lab.ReadCounters(ctx)
	if err != nil {
		return in, err
	}
	serverEvents, err := pipereport.ReadLog(serverLog)
	if err != nil {
		return in, err
	}
	return pipereport.Input{
		Profile: o.Profile.Name, Grade: o.Profile.Grade(), About: o.Profile.About,
		Client: o.Client, Seed: o.Seed,
		Switched: switched, End: end,
		Phone: phoneEvents, Server: serverEvents,
		WireDown: after.Down - before.Down, WireUp: after.Up - before.Up,
	}, nil
}

// collect records the driver's events until the time until. The driver
// exiting before then fails the run.
func collect(ctx context.Context, driver *phone.Driver, record func(phone.Event) error, until time.Time) error {
	timer := time.NewTimer(time.Until(until))
	defer timer.Stop()
	for {
		select {
		case ev, ok := <-driver.Events:
			if !ok {
				return fmt.Errorf("driver exited during the run: %v", driver.Wait())
			}
			if err := record(ev); err != nil {
				return err
			}
		case <-timer.C:
			return nil
		case <-ctx.Done():
			return context.Cause(ctx)
		}
	}
}

// drainDriver records the driver's last events until it exits.
func drainDriver(ctx context.Context, driver *phone.Driver, record func(phone.Event) error) error {
	timeout := time.After(2 * time.Minute)
	for {
		select {
		case ev, ok := <-driver.Events:
			if !ok {
				return driver.Wait()
			}
			if err := record(ev); err != nil {
				return err
			}
		case <-timeout:
			return errors.New("driver did not exit within 2 minutes of the stop")
		case <-ctx.Done():
			return context.Cause(ctx)
		}
	}
}
