package run

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"sync"
	"syscall"
	"time"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
	"github.com/djosh34/t3-optimizations/lab/internal/profile"
	"github.com/djosh34/t3-optimizations/lab/internal/report"
	"github.com/djosh34/t3-optimizations/lab/internal/t3server"
	"github.com/djosh34/t3-optimizations/lab/internal/userproc"
)

// StoryOptions select a story run: both clients side by side on one seed's
// network story (issue #58).
type StoryOptions struct {
	Seed   int64
	OutDir string
	// Name is the run's directory under OutDir; by default s<seed>.
	Name   string
	LabDir string
	LabExe string
	Node   string
}

// The prompts of the two clients: the measured prompt, told apart so the
// agent's log says which turn is whose.
func storyPrompt(client string) string { return agent.MeasuredPrompt + " (" + client + ")" }

// RunStory runs the untouched v0.1.7 web client and the t3opt web app at the
// same time on the same seed's story, against one stock t3 server: each
// client in its own lab (phone container, shaped link), the v0.1.7 client's
// front in the first lab, the server, the sidecar and the web app's front
// in the second, the two server namespaces joined by an unshaped bridge.
// The sidecar runs `t3 pair`, which finds the server on its own host. Both links follow the story in lockstep. It writes each client's
// report and story.json, and returns the headline.
func RunStory(ctx context.Context, o StoryOptions) (string, error) {
	u, err := userproc.Invoking()
	if err != nil {
		return "", err
	}
	repo := filepath.Dir(o.LabDir)
	staticDir := filepath.Join(u.Home, ".cache", "t3lab", "baseline-client-"+BaselineCommit[:12], "dist")
	if _, err := os.Stat(filepath.Join(staticDir, "index.html")); err != nil {
		return "", fmt.Errorf("no baseline client build at %s; run lab/scripts/build-baseline-client.sh as yourself first", staticDir)
	}
	sidecarBin, _, webDir, err := sidecarBinaries(o.LabDir, true)
	if err != nil {
		return "", err
	}
	storyJSON, err := exec.CommandContext(ctx, filepath.Join(o.LabDir, "bin", "story"), strconv.FormatInt(o.Seed, 10)).Output()
	if err != nil {
		return "", fmt.Errorf("lab/bin/story %d (built by lab/scripts/build-sidecar.sh): %w", o.Seed, err)
	}
	p, err := profile.FromStory(storyJSON)
	if err != nil {
		return "", err
	}
	if err := netlab.Cleanup(ctx); err != nil {
		return "", fmt.Errorf("cleanup of old labs: %w", err)
	}
	var labs []*netlab.Lab
	defer func() {
		cleanupCtx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
		defer cancel()
		for _, lab := range labs {
			if err := lab.Delete(cleanupCtx); err != nil {
				fmt.Fprintf(os.Stderr, "lab: cleanup of lab %s: %v\n", lab.ID, err)
			}
		}
	}()
	for range 2 {
		lab, err := netlab.Claim()
		if err != nil {
			return "", err
		}
		labs = append(labs, lab)
		reaper := exec.Command(o.LabExe, "reap", "--id", lab.ID, "--pid", strconv.Itoa(os.Getpid()))
		reaper.SysProcAttr = &syscall.SysProcAttr{Setsid: true}
		if err := reaper.Start(); err != nil {
			return "", err
		}
		go func() {
			if err := reaper.Wait(); err != nil {
				fmt.Fprintf(os.Stderr, "lab: reaper: %v\n", err)
			}
		}()
	}

	runDir := filepath.Join(o.OutDir, cmpOr(o.Name, fmt.Sprintf("s%d", o.Seed)))
	if err := u.WriteFile(filepath.Join(runDir, "story-input.json"), storyJSON); err != nil {
		return "", err
	}
	fmt.Fprintf(os.Stderr, "lab: story seed %d (%s) in %s\n", o.Seed, p.About, runDir)
	clients := []string{"websocket", ClientWebapp}
	var scenarios []*scenario
	for i, client := range clients {
		scenarios = append(scenarios, &scenario{
			o: Options{Profile: p, Seed: o.Seed, Client: client, Kind: "warm", LabDir: o.LabDir, LabExe: o.LabExe, Node: o.Node},
			u: u, lab: labs[i], runDir: filepath.Join(runDir, client), deadline: p.Deadline(),
		})
	}
	st := &storyRun{o: o, p: p, u: u, runDir: runDir, scenarios: scenarios}
	result, err := st.execute(ctx, repo, staticDir, sidecarBin, webDir)
	if err != nil {
		return "", err
	}
	result.Commit = buildCommit()
	for _, seg := range p.Phases {
		result.Phases = append(result.Phases, fmt.Sprintf("%s@%.0f", seg.Name, seg.At.Seconds()))
	}
	if err := result.Write(runDir, u.WriteFile); err != nil {
		return "", err
	}
	var parts []string
	for _, c := range result.Clients {
		parts = append(parts, fmt.Sprintf("%s %s, prompt %s, freeze %.0fs", c.Client, c.Status, fmtOpt(c.PromptConfirmationS), c.LongestFreezeS))
	}
	return fmt.Sprintf("story seed %d: %s", o.Seed, strings.Join(parts, "; ")), nil
}

func cmpOr(a, b string) string {
	if a != "" {
		return a
	}
	return b
}

func fmtOpt(v *float64) string {
	if v == nil {
		return "never"
	}
	return fmt.Sprintf("%.1fs", *v)
}

type storyRun struct {
	o         StoryOptions
	p         profile.Profile
	u         userproc.User
	runDir    string
	scenarios []*scenario
}

func (st *storyRun) execute(parent context.Context, repo, staticDir, sidecarBin, webDir string) (res report.StoryResult, err error) {
	ctx, fail := context.WithCancelCause(parent)
	defer func() {
		if err != nil && ctx.Err() != nil && parent.Err() == nil {
			err = context.Cause(ctx)
		}
		fail(nil)
	}()
	ws, app := st.scenarios[0], st.scenarios[1]
	var all []*processes
	defer func() {
		for _, p := range all {
			p.stop()
		}
	}()
	for _, s := range st.scenarios {
		// Made as the user before docker would make it as root.
		phoneDir := filepath.Join(s.runDir, "phone")
		if err := st.u.MkdirAll(phoneDir); err != nil {
			return res, err
		}
		procs, err := s.startLink(ctx, fail, phoneDir)
		if err != nil {
			return res, err
		}
		all = append(all, procs)
	}
	if err := ws.lab.Bridge(ctx, app.lab); err != nil {
		return res, err
	}
	wsProcs, procs := all[0], all[1]
	server, err := t3server.Start(procs.ctx, t3server.Config{
		User: st.u, NS: app.lab.ServerNS, LabID: app.lab.ID, T3Code: filepath.Join(repo, "t3code"), RunDir: st.runDir,
		LabExe: st.o.LabExe, Node: st.o.Node, CharsPerSecond: CharsPerSecond, Host: "0.0.0.0",
	})
	if err != nil {
		return res, err
	}
	procs.watch("t3 server", server.Exited())
	exited, err := ws.startLabProcess(wsProcs.ctx, ws.lab.ServerNS, "front.log", "front", "--listen", ":"+frontPort,
		"--upstream", fmt.Sprintf("%s:%d", netlab.BridgePeerAddr, t3server.Port), "--client-dir", staticDir)
	if err != nil {
		return res, err
	}
	wsProcs.watch("v0.1.7 client's front", exited)

	setupCtx, cancelSetup := context.WithTimeout(ctx, setupTimeout)
	defer cancelSetup()
	origin := "https://" + netlab.ServerAddr + ":" + frontPort
	wsThread, pairURL, err := server.Setup(setupCtx, origin)
	if err != nil {
		return res, err
	}
	appThread, err := server.StartThread(setupCtx, "Lab thread (web app)")
	if err != nil {
		return res, err
	}
	var codes []string
	for range 2 {
		code, err := server.PairingCode(setupCtx)
		if err != nil {
			return res, err
		}
		codes = append(codes, code)
	}
	appCode, watchCode := codes[0], codes[1]

	exited, err = app.startProcess(procs.ctx, app.lab.ServerNS, "sidecar.log", sidecarBin,
		"--t3-url", fmt.Sprintf("http://127.0.0.1:%d", t3server.Port),
		"--t3-base-dir", server.BaseDir(),
		"--t3-command", strings.Join(server.CLI(), " "),
		"--data-dir", filepath.Join(app.runDir, "sidecar"),
		"--http", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarHTTPPort),
		"--udp", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarUDPPort),
		"--web-dir", webDir)
	if err != nil {
		return res, err
	}
	procs.watch("sidecar", exited)
	exited, err = app.startLabProcess(procs.ctx, app.lab.ServerNS, "front.log", "front",
		"--listen", ":"+frontPort, "--upstream", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarHTTPPort))
	if err != nil {
		return res, err
	}
	procs.watch("web app's front", exited)

	// When the server releases text, as a client on its own host sees it.
	releases := filepath.Join(st.runDir, "releases.jsonl")
	exited, err = st.startWatch(procs.ctx, app, server, releases, watchCode, wsThread, appThread)
	if err != nil {
		return res, err
	}
	procs.watch("t3-watch", exited)

	reply := agent.Normalize(agent.ReplyText(agent.Script(agent.MeasuredPrompt, 0)))
	image := base64.StdEncoding.EncodeToString(agent.Picture())
	cfgs := []phone.Config{
		{Origin: origin, PairURL: pairURL, ThreadID: wsThread, Prompt: storyPrompt("websocket"),
			ImagePrompt: agent.ImagePrompt, Image: image, Reply: reply, Kind: "warm"},
		{Origin: origin, PairURL: origin + "/pair#token=" + appCode, ThreadID: appThread, Prompt: storyPrompt(ClientWebapp),
			ImagePrompt: agent.ImagePrompt, Image: image, Reply: reply, Kind: "warm", Webapp: true},
	}
	var drivers []*phone.Driver
	var recs []*recorder
	for i, s := range st.scenarios {
		d, err := phone.StartDriver(ctx, s.lab, st.u, "driver.mjs", cfgs[i], filepath.Join(s.runDir, "phone"), filepath.Join(s.runDir, "driver.log"))
		if err != nil {
			return res, err
		}
		drivers = append(drivers, d)
		rec, err := s.newRecorder()
		if err != nil {
			return res, err
		}
		defer rec.close()
		recs = append(recs, rec)
	}
	if err := eachClient(st.scenarios, func(i int, s *scenario) error { return s.waitReady(ctx, drivers[i], recs[i].record) }); err != nil {
		return res, err
	}

	load := startLoad(ctx)
	before := make([]netlab.Counters, len(st.scenarios))
	for i, s := range st.scenarios {
		if before[i], err = s.lab.ReadCounters(ctx); err != nil {
			return res, err
		}
	}
	switched := time.Now()
	labs := []*netlab.Lab{ws.lab, app.lab}
	for _, lab := range labs {
		if err := lab.Shape(ctx, st.p.Down, st.p.Up, st.o.Seed); err != nil {
			return res, err
		}
	}
	for _, d := range drivers {
		if err := d.Send("go"); err != nil {
			return res, err
		}
	}
	fmt.Fprintf(os.Stderr, "lab: story applied to both links, deadline %s\n", st.p.Deadline())
	measureCtx, abort := context.WithCancelCause(ctx)
	faultCtx, stopFaults := context.WithCancel(measureCtx)
	faultsDone := make(chan struct{})
	go func() {
		defer close(faultsDone)
		if err := injectFaults(faultCtx, labs, st.p, st.o.Seed, switched); err != nil {
			abort(fmt.Errorf("story: %w", err))
		}
	}()
	ends := make([]time.Time, len(st.scenarios))
	err = eachClient(st.scenarios, func(i int, s *scenario) error {
		var err error
		ends[i], err = s.measure(measureCtx, drivers[i], recs[i].record, switched)
		return err
	})
	stopFaults()
	<-faultsDone
	abort(nil)
	if err != nil {
		return res, err
	}
	res.Load = load()

	agentLog, err := readAgentLog(t3server.AgentLog(st.runDir))
	if err != nil {
		return res, err
	}
	rel, err := report.ReadReleases(releases)
	if err != nil {
		return res, err
	}
	res.Seed, res.Survival = st.o.Seed, strings.Contains(st.p.About, "survival")
	threads := []string{wsThread, appThread}
	paths := []string{report.PathNote, webappPath}
	for i, s := range st.scenarios {
		after, err := s.lab.ReadCounters(ctx)
		if err != nil {
			return res, err
		}
		in := report.Input{
			Profile: st.p.Name, Grade: st.p.Grade(), About: st.p.About,
			Client: s.o.Client, Kind: "warm", Seed: st.o.Seed, Deadline: s.deadline,
			Switched: switched, End: ends[i], Reply: reply,
			Agent: turnOf(agentLog, cfgs[i].Prompt), Phone: recs[i].events,
			WireDown: after.Down - before[i].Down, WireUp: after.Up - before[i].Up,
			Path: paths[i],
		}
		in.Commit = buildCommit()
		if err := report.Build(in).Write(s.runDir, st.u.WriteFile); err != nil {
			return res, err
		}
		res.Clients = append(res.Clients, report.BuildStoryClient(in, rel, threads[i]))
	}
	return res, nil
}

// startWatch runs t3-watch.mjs next to the server: its lines go to path,
// its errors to watch.log.
func (st *storyRun) startWatch(ctx context.Context, s *scenario, server *t3server.Server, path, code string, threads ...string) (<-chan error, error) {
	out, err := st.u.Create(path)
	if err != nil {
		return nil, err
	}
	log, err := st.u.Create(filepath.Join(st.runDir, "watch.log"))
	if err != nil {
		return nil, errors.Join(err, out.Close())
	}
	args := append([]string{filepath.Join(st.o.LabDir, "scripts", "t3-watch.mjs"), fmt.Sprintf("http://127.0.0.1:%d", t3server.Port), code}, threads...)
	cmd := st.u.Command(ctx, s.lab.ServerNS, []string{"PATH=/usr/bin:/bin"}, st.runDir, server.CLI()[0], args...)
	cmd.Stdout = out
	cmd.Stderr = log
	if err := cmd.Start(); err != nil {
		return nil, errors.Join(err, out.Close(), log.Close())
	}
	exited := make(chan error, 1)
	go func() { exited <- errors.Join(cmd.Wait(), out.Close(), log.Close()) }()
	return exited, nil
}

// eachClient runs f for every client at once and returns the first error.
func eachClient(scenarios []*scenario, f func(int, *scenario) error) error {
	errs := make([]error, len(scenarios))
	var wg sync.WaitGroup
	for i, s := range scenarios {
		wg.Add(1)
		go func() {
			defer wg.Done()
			errs[i] = f(i, s)
		}()
	}
	wg.Wait()
	return errors.Join(errs...)
}

// turnOf keeps the agent's events of the session whose turn had this
// prompt: each client has its own thread, and so its own session.
func turnOf(log []agent.LogEvent, prompt string) []agent.LogEvent {
	session := ""
	for _, ev := range log {
		if ev.Ev == "prompt" && ev.Prompt == prompt {
			session = ev.Session
		}
	}
	var out []agent.LogEvent
	for _, ev := range log {
		if session != "" && ev.Session == session {
			out = append(out, ev)
		}
	}
	return out
}

// startLoad samples the host's load every 5 s until the returned function
// is called, which returns what it saw.
func startLoad(ctx context.Context) func() report.Load {
	l := report.Load{CPUs: runtime.NumCPU(), MinMemAvailMB: -1}
	var mu sync.Mutex
	var sum float64
	n := 0
	sample := func() {
		load, mem := readLoad()
		mu.Lock()
		defer mu.Unlock()
		sum += load
		n++
		l.MaxLoad1 = max(l.MaxLoad1, load)
		if mem >= 0 && (l.MinMemAvailMB < 0 || mem < l.MinMemAvailMB) {
			l.MinMemAvailMB = mem
		}
	}
	ctx, stop := context.WithCancel(ctx)
	done := make(chan struct{})
	go func() {
		defer close(done)
		ticker := time.NewTicker(5 * time.Second)
		defer ticker.Stop()
		for {
			sample()
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
			}
		}
	}()
	return func() report.Load {
		stop()
		<-done
		mu.Lock()
		defer mu.Unlock()
		if n > 0 {
			l.MeanLoad1 = sum / float64(n)
		}
		return l
	}
}

// readLoad reads the 1-minute load average and the available memory in
// MB; -1 for what it could not read.
func readLoad() (float64, int64) {
	load, mem := -1.0, int64(-1)
	if data, err := os.ReadFile("/proc/loadavg"); err == nil {
		if f := strings.Fields(string(data)); len(f) > 0 {
			if v, err := strconv.ParseFloat(f[0], 64); err == nil {
				load = v
			}
		}
	}
	if data, err := os.ReadFile("/proc/meminfo"); err == nil {
		for _, line := range strings.Split(string(data), "\n") {
			if rest, ok := strings.CutPrefix(line, "MemAvailable:"); ok {
				if f := strings.Fields(rest); len(f) > 0 {
					if kb, err := strconv.ParseInt(f[0], 10, 64); err == nil {
						mem = kb / 1024
					}
				}
			}
		}
	}
	return load, mem
}
