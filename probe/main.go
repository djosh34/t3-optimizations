// Throwaway probe for djosh34/t3-optimizations#22: which UDP-capable browser
// transports reach this host from an iPhone over Tailscale.
//
// One process:
//   - TCP  :6443  HTTPS/1.1 (page, service worker, signaling, results, WebSocket over TCP)
//   - UDP  :6443  HTTP/3 (same routes, plus WebTransport at /wt and WebSocket over H3 at /ws)
//   - UDP  :6444  WebRTC ICE (pion, ICE-lite, tailscale0 addresses only)
package main

import (
	"bytes"
	"context"
	"crypto/rand"
	"crypto/tls"
	_ "embed"
	"encoding/hex"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"log"
	"net"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/gobwas/ws"
	"github.com/gobwas/ws/wsutil"
	"github.com/pion/ice/v4"
	"github.com/pion/webrtc/v4"
	"github.com/quic-go/quic-go"
	"github.com/quic-go/quic-go/http3"
	"github.com/quic-go/webtransport-go"
)

//go:embed static/index.html
var indexHTML []byte

//go:embed static/sw.js
var swJS []byte

//go:embed static/manifest.json
var manifestJSON []byte

//go:embed static/icon.png
var iconPNG []byte

var (
	flagHost    = flag.String("host", "lima-opencode-vm.tail2fc05a.ts.net", "tailnet DNS name (cert name)")
	flagIP      = flag.String("ip", "100.119.164.40", "tailnet IPv4 to bind HTTPS/H3 to")
	flagIP6     = flag.String("ip6", "fd7a:115c:a1e0::3a01:a4b2", "tailnet IPv6 to bind HTTPS/H3 to (empty: none)")
	flagPort    = flag.Int("port", 6443, "HTTPS (TCP) and HTTP/3 (UDP) port")
	flagICEPort = flag.Int("ice-port", 6444, "WebRTC ICE UDP port")
	flagIface   = flag.String("iface", "tailscale0", "interface for WebRTC host candidates")
	flagCert    = flag.String("cert", "", "TLS cert file")
	flagKey     = flag.String("key", "", "TLS key file")
	flagIssue   = flag.String("issue", "22", "GitHub issue number for result comments")
	flagRepo    = flag.String("repo", "djosh34/t3-optimizations", "GitHub repo for result comments")
	flagPost    = flag.Bool("post", true, "post each result as a GitHub comment")
	flagOut     = flag.String("out", "results", "directory for raw result JSON")
	flagRender  = flag.String("render", "", "print the comment for a saved result JSON and exit")
)

var altSvc string

func main() {
	flag.Parse()
	if *flagRender != "" {
		b, err := os.ReadFile(*flagRender)
		if err != nil {
			log.Fatal(err)
		}
		var res map[string]any
		if err := json.Unmarshal(b, &res); err != nil {
			log.Fatal(err)
		}
		fmt.Print(renderComment(res))
		return
	}
	altSvc = fmt.Sprintf(`h3=":%d"; ma=86400`, *flagPort)
	if err := os.MkdirAll(*flagOut, 0o755); err != nil {
		log.Fatal(err)
	}
	cert, err := tls.LoadX509KeyPair(*flagCert, *flagKey)
	if err != nil {
		log.Fatalf("load cert: %v", err)
	}
	api, err := newWebRTCAPI()
	if err != nil {
		log.Fatalf("webrtc: %v", err)
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/", serveStatic)
	mux.HandleFunc("/ping", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cache-Control", "no-store")
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]any{"proto": r.Proto, "t": time.Now().UnixMilli()})
	})
	mux.HandleFunc("/rtc/offer", func(w http.ResponseWriter, r *http.Request) { handleOffer(api, w, r) })
	mux.HandleFunc("/result", handleResult)

	wtServer := &webtransport.Server{CheckOrigin: func(*http.Request) bool { return true }}
	mux.HandleFunc("/wt", func(w http.ResponseWriter, r *http.Request) { handleWT(wtServer, w, r) })
	mux.HandleFunc("/ws", handleWS)

	h3 := &http3.Server{
		Handler:   withHeaders(mux),
		TLSConfig: http3.ConfigureTLSConfig(&tls.Config{Certificates: []tls.Certificate{cert}}),
		QUICConfig: &quic.Config{
			EnableDatagrams:                  true,
			EnableStreamResetPartialDelivery: true,
			MaxIdleTimeout:                   60 * time.Second,
			KeepAlivePeriod:                  15 * time.Second,
		},
	}
	webtransport.ConfigureHTTP3Server(h3)
	wtServer.H3 = h3

	ips := []string{*flagIP}
	if *flagIP6 != "" {
		ips = append(ips, *flagIP6)
	}
	for _, ip := range ips {
		addr := net.JoinHostPort(ip, fmt.Sprint(*flagPort))
		udp, err := net.ListenPacket("udp", addr)
		if err != nil {
			log.Fatalf("listen udp %s: %v", addr, err)
		}
		go func() { log.Fatal(wtServer.Serve(udp)) }()
		tcpSrv := &http.Server{
			Addr:      addr,
			Handler:   withHeaders(mux),
			TLSConfig: &tls.Config{Certificates: []tls.Certificate{cert}, NextProtos: []string{"http/1.1"}},
			// HTTP/1.1 only on TCP, so the protocol each request used is unambiguous.
			TLSNextProto: map[string]func(*http.Server, *tls.Conn, http.Handler){},
		}
		go func() { log.Fatal(tcpSrv.ListenAndServeTLS("", "")) }()
		log.Printf("listening https+h3 on %s", addr)
	}
	log.Printf("open https://%s:%d/", *flagHost, *flagPort)
	select {}
}

func withHeaders(h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Alt-Svc", altSvc)
		h.ServeHTTP(w, r)
	})
}

func serveStatic(w http.ResponseWriter, r *http.Request) {
	switch r.URL.Path {
	case "/", "/index.html":
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		w.Header().Set("Cache-Control", "no-cache")
		w.Write(indexHTML)
	case "/sw.js":
		w.Header().Set("Content-Type", "text/javascript")
		w.Header().Set("Cache-Control", "no-cache")
		w.Write(swJS)
	case "/manifest.json":
		w.Header().Set("Content-Type", "application/manifest+json")
		w.Write(manifestJSON)
	case "/icon.png", "/apple-touch-icon.png":
		w.Header().Set("Content-Type", "image/png")
		w.Write(iconPNG)
	default:
		http.NotFound(w, r)
	}
}

// ---------- WebRTC ----------

var rtcSessions sync.Map // id -> *rtcSession

type rtcSession struct {
	mu       sync.Mutex
	Test     string   `json:"test"`
	Events   []string `json:"events"`
	Selected string   `json:"selectedPair,omitempty"`
	Echoed   int      `json:"echoed"`
	start    time.Time
}

func (s *rtcSession) logf(format string, a ...any) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.Events = append(s.Events, fmt.Sprintf("+%dms ", time.Since(s.start).Milliseconds())+fmt.Sprintf(format, a...))
}

func newWebRTCAPI() (*webrtc.API, error) {
	keepIP := func(ip net.IP) bool {
		// Tailscale ranges only: 100.64.0.0/10 and fd7a:115c:a1e0::/48.
		_, cgnat, _ := net.ParseCIDR("100.64.0.0/10")
		_, ts6, _ := net.ParseCIDR("fd7a:115c:a1e0::/48")
		return cgnat.Contains(ip) || ts6.Contains(ip)
	}
	udpMux, err := ice.NewMultiUDPMuxFromPort(*flagICEPort,
		ice.UDPMuxFromPortWithInterfaceFilter(func(name string) bool { return name == *flagIface }),
		ice.UDPMuxFromPortWithIPFilter(keepIP),
	)
	if err != nil {
		return nil, err
	}
	se := webrtc.SettingEngine{}
	se.SetLite(true)
	se.SetICEUDPMux(udpMux)
	se.SetNetworkTypes([]webrtc.NetworkType{webrtc.NetworkTypeUDP4, webrtc.NetworkTypeUDP6})
	se.SetInterfaceFilter(func(name string) bool { return name == *flagIface })
	se.SetIPFilter(keepIP)
	se.SetICEMulticastDNSMode(ice.MulticastDNSModeDisabled)
	return webrtc.NewAPI(webrtc.WithSettingEngine(se)), nil
}

func handleOffer(api *webrtc.API, w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "POST only", 405)
		return
	}
	var req struct {
		ID   string `json:"id"`
		Test string `json:"test"`
		SDP  string `json:"sdp"`
	}
	if err := json.NewDecoder(io.LimitReader(r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, err.Error(), 400)
		return
	}
	sess := &rtcSession{Test: req.Test, start: time.Now()}
	rtcSessions.Store(req.ID, sess)
	time.AfterFunc(10*time.Minute, func() { rtcSessions.Delete(req.ID) })

	pc, err := api.NewPeerConnection(webrtc.Configuration{})
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	time.AfterFunc(2*time.Minute, func() { pc.Close() })
	pc.OnICEConnectionStateChange(func(st webrtc.ICEConnectionState) {
		sess.logf("ice %s", st)
		if st == webrtc.ICEConnectionStateConnected {
			if sctp := pc.SCTP(); sctp != nil {
				if pair, err := sctp.Transport().ICETransport().GetSelectedCandidatePair(); err == nil && pair != nil {
					sess.mu.Lock()
					sess.Selected = pair.String()
					sess.mu.Unlock()
				}
			}
		}
	})
	pc.OnConnectionStateChange(func(st webrtc.PeerConnectionState) { sess.logf("pc %s", st) })
	pc.OnDataChannel(func(dc *webrtc.DataChannel) {
		var mr, ml string
		if v := dc.MaxRetransmits(); v != nil {
			mr = fmt.Sprint(*v)
		}
		if v := dc.MaxPacketLifeTime(); v != nil {
			ml = fmt.Sprint(*v)
		}
		sess.logf("datachannel %q ordered=%v maxRetransmits=%s maxPacketLifeTime=%s", dc.Label(), dc.Ordered(), mr, ml)
		dc.OnMessage(func(m webrtc.DataChannelMessage) {
			sess.mu.Lock()
			sess.Echoed++
			sess.mu.Unlock()
			if m.IsString {
				dc.SendText(string(m.Data))
			} else {
				dc.Send(m.Data)
			}
		})
	})
	if err := pc.SetRemoteDescription(webrtc.SessionDescription{Type: webrtc.SDPTypeOffer, SDP: req.SDP}); err != nil {
		http.Error(w, "set remote: "+err.Error(), 400)
		pc.Close()
		return
	}
	answer, err := pc.CreateAnswer(nil)
	if err != nil {
		http.Error(w, err.Error(), 500)
		pc.Close()
		return
	}
	done := webrtc.GatheringCompletePromise(pc)
	if err := pc.SetLocalDescription(answer); err != nil {
		http.Error(w, err.Error(), 500)
		pc.Close()
		return
	}
	select {
	case <-done:
	case <-time.After(5 * time.Second):
	}
	sess.logf("answer sent via %s from %s", r.Proto, r.RemoteAddr)
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"sdp": pc.LocalDescription().SDP})
}

// ---------- WebTransport ----------

var wtInfo sync.Map // id -> map[string]any

func handleWT(s *webtransport.Server, w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	info := map[string]any{"proto": r.Proto, "remote": r.RemoteAddr}
	var mu sync.Mutex
	set := func(k string, v any) { mu.Lock(); info[k] = v; mu.Unlock() }
	if st, ok := w.(http3.Settingser); ok {
		select {
		case <-st.ReceivedSettings():
			cs := st.Settings()
			set("clientH3Datagram", cs.EnableDatagrams)
			set("clientExtendedConnect", cs.EnableExtendedConnect)
		case <-time.After(3 * time.Second):
			set("clientSettings", "not received in 3s")
		}
	}
	sess, err := s.Upgrade(w, r)
	if err != nil {
		set("upgradeError", err.Error())
		wtInfo.Store(id, info)
		log.Printf("wt upgrade: %v", err)
		w.WriteHeader(500)
		return
	}
	wtInfo.Store(id, info)
	ctx := sess.Context()
	var dgIn, streams int
	go func() {
		for {
			b, err := sess.ReceiveDatagram(ctx)
			if err != nil {
				return
			}
			mu.Lock()
			dgIn++
			info["datagramsEchoed"] = dgIn
			mu.Unlock()
			sess.SendDatagram(b)
		}
	}()
	go func() {
		for {
			str, err := sess.AcceptStream(ctx)
			if err != nil {
				return
			}
			mu.Lock()
			streams++
			info["streamsEchoed"] = streams
			mu.Unlock()
			go func() {
				b, _ := io.ReadAll(io.LimitReader(str, 64<<10))
				str.Write(b)
				str.Close()
			}()
		}
	}()
}

// ---------- WebSocket (HTTP/1.1 upgrade on TCP, RFC 9220 extended CONNECT on HTTP/3) ----------

var wsInfo sync.Map // id -> string

func handleWS(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var rw io.ReadWriter
	if r.Method == http.MethodConnect && r.Proto == "websocket" {
		// RFC 9220: no key/accept handshake; the request stream carries frames.
		hs, ok := w.(http3.HTTPStreamer)
		if !ok {
			http.Error(w, "no stream", 500)
			return
		}
		w.WriteHeader(http.StatusOK)
		w.(http.Flusher).Flush()
		str := hs.HTTPStream()
		defer str.Close()
		rw = str
		wsInfo.Store(id, "HTTP/3 extended CONNECT (RFC 9220)")
	} else {
		conn, _, _, err := ws.UpgradeHTTP(r, w)
		if err != nil {
			return
		}
		defer conn.Close()
		rw = conn
		wsInfo.Store(id, r.Proto+" Upgrade over TCP")
	}
	for {
		msg, op, err := wsutil.ReadClientData(rw)
		if err != nil {
			return
		}
		if err := wsutil.WriteServerMessage(rw, op, msg); err != nil {
			return
		}
	}
}

// ---------- results ----------

var posted sync.Map // run id -> true

func handleResult(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "POST only", 405)
		return
	}
	body, err := io.ReadAll(io.LimitReader(r.Body, 1<<20))
	if err != nil {
		http.Error(w, err.Error(), 400)
		return
	}
	var res map[string]any
	if err := json.Unmarshal(body, &res); err != nil {
		http.Error(w, err.Error(), 400)
		return
	}
	runID, _ := res["runId"].(string)
	if runID == "" {
		runID = randID()
	}
	remoteIP, _, _ := net.SplitHostPort(r.RemoteAddr)
	server := map[string]any{
		"receivedAt":    time.Now().UTC().Format(time.RFC3339),
		"resultProto":   r.Proto,
		"remoteIP":      remoteIP,
		"tailscalePath": tailscalePath(remoteIP),
	}
	for _, key := range []string{"rtcIds", "wtIds", "wsIds"} {
		ids, _ := res[key].(map[string]any)
		out := map[string]any{}
		for name, v := range ids {
			id, _ := v.(string)
			switch key {
			case "rtcIds":
				if s, ok := rtcSessions.Load(id); ok {
					sess := s.(*rtcSession)
					sess.mu.Lock()
					out[name] = map[string]any{"events": append([]string(nil), sess.Events...), "selectedPair": sess.Selected, "echoed": sess.Echoed}
					sess.mu.Unlock()
				}
			case "wtIds":
				if v, ok := wtInfo.Load(id); ok {
					out[name] = v
				}
			case "wsIds":
				if v, ok := wsInfo.Load(id); ok {
					out[name] = v
				}
			}
		}
		server[strings.TrimSuffix(key, "Ids")] = out
	}
	res["server"] = server
	annotate(res, server)
	full, _ := json.MarshalIndent(res, "", "  ")
	fname := filepath.Join(*flagOut, time.Now().UTC().Format("20060102T150405Z")+"-"+sanitize(runID)+".json")
	os.WriteFile(fname, full, 0o644)
	log.Printf("result %s saved to %s", runID, fname)

	status := "saved"
	dry, _ := res["dryRun"].(bool)
	if *flagPost && !dry {
		if _, dup := posted.LoadOrStore(runID, true); dup {
			status = "saved (already posted)"
		} else if err := postComment(res); err != nil {
			log.Printf("post comment: %v", err)
			status = "saved, GitHub post failed: " + err.Error()
			posted.Delete(runID)
		} else {
			status = "saved and posted to #" + *flagIssue
		}
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": status})
}

func sanitize(s string) string {
	return strings.Map(func(r rune) rune {
		if r >= 'a' && r <= 'z' || r >= 'A' && r <= 'Z' || r >= '0' && r <= '9' || r == '-' {
			return r
		}
		return '_'
	}, s)
}

func randID() string {
	b := make([]byte, 6)
	rand.Read(b)
	return hex.EncodeToString(b)
}

// tailscalePath reports how tailscaled reaches the peer with this tailnet IP.
// Public endpoint addresses are redacted (the comments land on a public repo).
func tailscalePath(ip string) map[string]any {
	out, err := exec.Command("tailscale", "status", "--json").Output()
	if err != nil {
		return map[string]any{"error": err.Error()}
	}
	var st struct {
		Peer map[string]struct {
			HostName     string
			OS           string
			TailscaleIPs []string
			CurAddr      string
			Relay        string
			PeerRelay    string
			RxBytes      int64
			TxBytes      int64
		}
	}
	if err := json.Unmarshal(out, &st); err != nil {
		return map[string]any{"error": err.Error()}
	}
	if ip == *flagIP || ip == *flagIP6 {
		return map[string]any{"path": "same host (local test from the server itself, not a phone)"}
	}
	for _, p := range st.Peer {
		for _, tip := range p.TailscaleIPs {
			if tip != ip {
				continue
			}
			path := "DERP relay (" + p.Relay + "), tunnel rides TCP"
			if p.PeerRelay != "" {
				path = "peer relay (UDP)"
			}
			if p.CurAddr != "" {
				path = "direct UDP to " + redactEndpoint(p.CurAddr)
			}
			return map[string]any{"peer": p.HostName, "os": p.OS, "path": path, "derpHome": p.Relay}
		}
	}
	return map[string]any{"path": "peer not found in tailscale status (request did not come over the tailnet?)"}
}

func redactEndpoint(hostport string) string {
	host, _, err := net.SplitHostPort(hostport)
	if err != nil {
		return "unknown endpoint"
	}
	ip := net.ParseIP(host)
	if ip == nil {
		return "unknown endpoint"
	}
	if ip.IsPrivate() {
		return "private/LAN endpoint " + hostport + " (likely same Wi-Fi/LAN)"
	}
	if ip.To4() != nil {
		return "public IPv4 endpoint (redacted)"
	}
	return "public IPv6 endpoint (redacted)"
}

func postComment(res map[string]any) error {
	body := renderComment(res)
	if len(body) > 60000 {
		body = body[:60000] + "\n…(truncated; full JSON is on the server)\n"
	}
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "gh", "issue", "comment", *flagIssue, "-R", *flagRepo, "--body-file", "-")
	cmd.Stdin = strings.NewReader(body)
	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("%v: %s", err, strings.TrimSpace(stderr.String()))
	}
	return nil
}

// annotate adds the server's view to each test summary and fills in a
// network guess when the owner did not tag the run.
func annotate(res, server map[string]any) {
	env, _ := res["env"].(map[string]any)
	tp, _ := server["tailscalePath"].(map[string]any)
	if env != nil && str(env["network"]) == "" {
		path := str(tp["path"])
		switch {
		case strings.Contains(path, "LAN endpoint"):
			env["network"] = "untagged; guess Wi-Fi (phone reached via a private LAN endpoint)"
		case strings.Contains(path, "same host"):
			env["network"] = "local test on the server"
		case path != "":
			env["network"] = "untagged; guess not on the server's LAN (cellular or other Wi-Fi)"
		}
	}
	rtc, _ := server["rtc"].(map[string]any)
	wt, _ := server["wt"].(map[string]any)
	wsv, _ := server["ws"].(map[string]any)
	tests, _ := res["tests"].([]any)
	for _, t := range tests {
		m, _ := t.(map[string]any)
		name := str(m["name"])
		var extra string
		switch {
		case strings.HasPrefix(name, "1."):
			extra = rtcNote(rtc["rtc-maxRetransmits0"])
		case strings.HasPrefix(name, "2."):
			extra = rtcNote(rtc["rtc-maxPacketLifeTime500"])
		case strings.HasPrefix(name, "3."):
			if v, ok := wt["WebTransport"].(map[string]any); ok {
				extra = fmt.Sprintf("server: client SETTINGS H3_DATAGRAM=%v, extended CONNECT=%v", v["clientH3Datagram"], v["clientExtendedConnect"])
				if e := str(v["upgradeError"]); e != "" {
					extra += ", upgrade error: " + e
				}
				if n := v["streamsEchoed"]; n != nil {
					extra += fmt.Sprintf(", streams echoed %v", n)
				}
				if n := v["datagramsEchoed"]; n != nil {
					extra += fmt.Sprintf(", datagrams echoed %v", n)
				}
			} else {
				extra = "server: no WebTransport session reached the server"
			}
		case strings.HasPrefix(name, "4."):
			if v := str(wsv["WebSocket"]); v != "" {
				extra = "server: WebSocket arrived via " + v
				if strings.Contains(v, "HTTP/3") {
					m["status"] = "ok (H3)"
				} else if str(m["status"]) == "ok" {
					m["status"] = "ok, TCP only"
				}
			} else {
				extra = "server: no WebSocket reached the server"
			}
		}
		if extra != "" {
			m["summary"] = str(m["summary"]) + "\n" + extra
		}
	}
}

func rtcNote(v any) string {
	m, ok := v.(map[string]any)
	if !ok {
		return "server: no WebRTC session recorded"
	}
	ev, _ := m["events"].([]string)
	return fmt.Sprintf("server: selected pair %q, echoed %v; %s", str(m["selectedPair"]), m["echoed"], strings.Join(ev, ", "))
}
