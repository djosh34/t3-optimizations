package run

import (
	"context"
	"encoding/base64"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/djosh34/t3-optimizations/lab/internal/agent"
	"github.com/djosh34/t3-optimizations/lab/internal/netlab"
	"github.com/djosh34/t3-optimizations/lab/internal/phone"
	"github.com/djosh34/t3-optimizations/lab/internal/report"
	"github.com/djosh34/t3-optimizations/lab/internal/t3server"
)

// ClientSidecarNative is the sync protocol on the WebRTC data channel to the
// sidecar (crates/sidecar), with the native phone of crates/native-phone in
// the web app's place.
const ClientSidecarNative = "sidecar-native"

// ClientWebapp is the home-screen web app (t3code/apps/web/src/t3opt) in
// WebKit, served by the sidecar behind the lab's HTTPS front, on the same
// data channel.
const ClientWebapp = "webapp"

// The sidecar's ports in the server namespace.
const (
	sidecarHTTPPort = 8480
	sidecarUDPPort  = 12444
)

const sidecarPath = "WebRTC data channel (UDP, str0m with the patched SCTP layer) over a veth pair " +
	"in the lab; the phone is crates/native-phone, not WebKit. In production the UDP goes through Tailscale."

const webappPath = "WebRTC data channel from WebKit to the sidecar (UDP, str0m with the patched SCTP layer) " +
	"over a veth pair in the lab; the app's HTTPS goes through the lab front. In production the UDP goes through Tailscale."

// sidecarBinaries returns the sidecar and the native phone, built by
// lab/scripts/build-sidecar.sh, and for the web app its build from
// lab/scripts/build-webapp.sh.
func sidecarBinaries(labDir string, webapp bool) (sidecar, native, web string, err error) {
	sidecar = filepath.Join(labDir, "bin", "sidecar")
	native = filepath.Join(labDir, "bin", "native-phone")
	need := []string{sidecar, native}
	if webapp {
		web = filepath.Join(labDir, "bin", "webapp")
		need = []string{sidecar, filepath.Join(web, "sw.js")}
	}
	for _, path := range need {
		if _, err := os.Stat(path); err != nil {
			return "", "", "", fmt.Errorf("no %s; run lab/scripts/build-sidecar.sh and lab/scripts/build-webapp.sh as yourself first", path)
		}
	}
	return sidecar, native, web, nil
}

// executeSidecar runs a t3 scenario through the sidecar: the forked server
// and the sidecar in the server namespace, the native phone or the web app
// (webDir set) in the phone container.
func (s *scenario) executeSidecar(parent context.Context, repo, phoneDir, sidecarBin, nativeBin, webDir string) (in report.Input, err error) {
	o, u, lab := s.o, s.u, s.lab
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

	soak := 0
	if o.Profile.Soak {
		soak = soakParts
	}
	server, err := t3server.Start(procs.ctx, t3server.Config{
		User: u, NS: lab.ServerNS, LabID: lab.ID, T3Code: filepath.Join(repo, "t3code"), RunDir: s.runDir,
		LabExe: o.LabExe, Node: o.Node, CharsPerSecond: CharsPerSecond, SoakParts: soak,
	})
	if err != nil {
		return in, err
	}
	procs.watch("t3 server", server.Exited())
	origin := fmt.Sprintf("http://%s:%d", netlab.ServerAddr, sidecarHTTPPort)
	setupCtx, cancelSetup := context.WithTimeout(ctx, setupTimeout)
	defer cancelSetup()
	threadID, pairURL, err := server.Setup(setupCtx, origin)
	if err != nil {
		return in, err
	}
	// The pairing URL carries a one-time code, which the phone gives the
	// sidecar, as the web app does.
	_, code, ok := strings.Cut(pairURL, "#token=")
	if !ok {
		return in, fmt.Errorf("no code in the pairing URL %q", pairURL)
	}

	args := []string{
		"--t3-url", "http://127.0.0.1:" + strconv.Itoa(t3server.Port),
		"--t3-base-dir", server.BaseDir(),
		"--t3-command", strings.Join(server.CLI(), " "),
		"--data-dir", filepath.Join(s.runDir, "sidecar"),
		"--http", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarHTTPPort),
		"--udp", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarUDPPort),
	}
	if webDir != "" {
		args = append(args, "--web-dir", webDir)
	}
	exited, err := s.startProcess(procs.ctx, lab.ServerNS, "sidecar.log", sidecarBin, args...)
	if err != nil {
		return in, err
	}
	procs.watch("sidecar", exited)
	reply := agent.Normalize(agent.ReplyText(agent.Script(agent.MeasuredPrompt, soak)))

	if webDir != "" {
		// The app needs a secure context (service worker, WebCrypto): the
		// lab front gives the sidecar HTTPS, as tailscale serve does.
		exited, err := s.startLabProcess(procs.ctx, lab.ServerNS, "front.log", "front",
			"--listen", ":"+frontPort, "--upstream", fmt.Sprintf("%s:%d", netlab.ServerAddr, sidecarHTTPPort))
		if err != nil {
			return in, err
		}
		procs.watch("front", exited)
		front := "https://" + netlab.ServerAddr + ":" + frontPort
		driver, err := phone.StartDriver(ctx, lab, u, "driver.mjs", phone.Config{
			Origin: front, PairURL: front + "/pair#token=" + code, ThreadID: threadID,
			Prompt: agent.MeasuredPrompt, ImagePrompt: agent.ImagePrompt,
			Image: base64.StdEncoding.EncodeToString(agent.Picture()),
			Reply: reply, Kind: o.Kind, Webapp: true,
		}, phoneDir, filepath.Join(s.runDir, "driver.log"))
		if err != nil {
			return in, err
		}
		return s.drive(ctx, driver, reply, webappPath)
	}

	if err := u.Install(nativeBin, filepath.Join(phoneDir, "native-phone")); err != nil {
		return in, err
	}
	driver, err := phone.StartNative(ctx, lab, u, "native-phone", phone.NativeConfig{
		Origin: origin, Code: code, ThreadID: threadID,
		Prompt: agent.MeasuredPrompt, Reply: reply, Kind: o.Kind, Local: netlab.PhoneAddr,
	}, phoneDir, filepath.Join(s.runDir, "driver.log"))
	if err != nil {
		return in, err
	}
	return s.drive(ctx, driver, reply, sidecarPath)
}
