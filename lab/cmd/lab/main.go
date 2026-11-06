// Command lab is the chaos lab. See lab/README.md.
package main

import (
	"context"
	"errors"
	"flag"
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"syscall"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/front"
	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/profile"
	"github.com/djosh34/t3-optimizations/lab/internal/report"
	"github.com/djosh34/t3-optimizations/lab/internal/run"
	"github.com/djosh34/t3-optimizations/lab/internal/tunnel"
)

const usage = `usage:
  sudo ./bin/lab run --profile <name> --seed <n> --client <client> [flags]
  sudo ./bin/lab cleanup
  ./bin/lab profiles
  ./bin/lab summary <reports dir>

The lab runs these itself:
  lab fakeagent   the scripted ACP agent the t3 server starts
  lab front       the HTTPS origin in the server namespace
  lab tunnel      one end of the TCP outer path
`

func main() {
	if len(os.Args) < 2 {
		fmt.Fprint(os.Stderr, usage)
		os.Exit(2)
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	err := dispatch(ctx, os.Args[1], os.Args[2:])
	stop()
	if err != nil {
		fmt.Fprintf(os.Stderr, "lab: %v\n", err)
		os.Exit(1)
	}
}

func dispatch(ctx context.Context, cmd string, args []string) error {
	switch cmd {
	case "run":
		return runCmd(ctx, args)
	case "cleanup":
		return netlab.Cleanup(ctx)
	case "profiles":
		for _, p := range profile.All {
			fmt.Printf("%-10s %-10s %s\n", p.Name, p.Grade(), p.About)
		}
		return nil
	case "summary":
		if len(args) != 1 {
			return errors.New("usage: lab summary <reports dir>")
		}
		return report.Summary(args[0])
	case "fakeagent":
		return fakeAgentCmd(ctx, args)
	case "front":
		return frontCmd(ctx, args)
	case "tunnel":
		return tunnelCmd(ctx, args)
	}
	fmt.Fprint(os.Stderr, usage)
	return fmt.Errorf("unknown command %q", cmd)
}

func runCmd(ctx context.Context, args []string) error {
	fs := flag.NewFlagSet("run", flag.ContinueOnError)
	name := fs.String("profile", "", "profile name; see `lab profiles`")
	seed := fs.Int64("seed", 1, "seed for netem and the fault schedule")
	client := fs.String("client", "websocket", "client: websocket or websocket-nodeflate")
	kind := fs.String("kind", "warm", "warm: start connected, then switch to the profile; cold: open the app under the profile")
	deadline := fs.Duration("deadline", 0, "override the profile's deadline (15m, 60m for the soak)")
	out := fs.String("out", "", "directory for the run's report and raw data (default lab/reports/runs)")
	node := fs.String("node", "", "node binary for the t3 server (default: node on root's PATH)")
	runName := fs.String("name", "", "the run's directory under --out (default: time, profile, client, kind, seed)")
	if err := fs.Parse(args); err != nil {
		return err
	}
	p, err := profile.Get(*name)
	if err != nil {
		return err
	}
	if *node == "" {
		if *node, err = exec.LookPath("node"); err != nil {
			return fmt.Errorf("no node on PATH; pass --node: %w", err)
		}
	}
	exe, err := os.Executable()
	if err != nil {
		return err
	}
	labDir := filepath.Dir(filepath.Dir(exe))
	if *out == "" {
		*out = filepath.Join(labDir, "reports", "runs")
	}
	// docker takes a relative path for a volume name, not a directory.
	if *out, err = filepath.Abs(*out); err != nil {
		return err
	}
	r, err := run.Run(ctx, run.Options{
		Profile:  p,
		Seed:     *seed,
		Client:   *client,
		Kind:     *kind,
		Deadline: *deadline,
		OutDir:   *out,
		Name:     *runName,
		LabDir:   labDir,
		LabExe:   exe,
		Node:     *node,
	})
	if err != nil {
		return err
	}
	fmt.Println(r.Headline())
	return nil
}

func fakeAgentCmd(ctx context.Context, args []string) error {
	fs := flag.NewFlagSet("fakeagent", flag.ContinueOnError)
	logPath := fs.String("log", "", "production log, appended to")
	rate := fs.Int("rate", 50, "reply characters per second")
	soakParts := fs.Int("soak-parts", 0, "extra paragraphs for the soak turn")
	if err := fs.Parse(args); err != nil {
		return err
	}
	log, err := os.OpenFile(*logPath, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0o644)
	if err != nil {
		return err
	}
	err = agent.Serve(ctx, agent.Config{CharsPerSecond: *rate, SoakParts: *soakParts, Log: log}, os.Stdin, os.Stdout)
	return errors.Join(err, log.Close())
}

func frontCmd(ctx context.Context, args []string) error {
	fs := flag.NewFlagSet("front", flag.ContinueOnError)
	listen := fs.String("listen", ":8443", "TLS address")
	upstream := fs.String("upstream", "", "t3 server address")
	noDeflate := fs.Bool("no-deflate", false, "remove the WebSocket compression offer")
	if err := fs.Parse(args); err != nil {
		return err
	}
	return front.Serve(ctx, front.Config{Listen: *listen, Upstream: *upstream, NoDeflate: *noDeflate})
}

func tunnelCmd(ctx context.Context, args []string) error {
	fs := flag.NewFlagSet("tunnel", flag.ContinueOnError)
	dev := fs.String("dev", "", "TUN device")
	listen := fs.String("listen", "", "accept the TCP connection here")
	connect := fs.String("connect", "", "dial the TCP connection here")
	if err := fs.Parse(args); err != nil {
		return err
	}
	if (*listen == "") == (*connect == "") {
		return errors.New("give exactly one of --listen and --connect")
	}
	return tunnel.Run(ctx, *dev, *listen, *connect)
}
