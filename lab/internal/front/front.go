// Package front is the lab's HTTPS origin. It terminates TLS with a fresh
// self-signed certificate and forwards requests to the t3 server, like
// `tailscale serve` does in production. HTTPS makes the page a secure
// context, so service workers work. With a client build it serves that build
// itself, so the stock t3 server needs no option to serve a web client.
package front

import (
	"compress/gzip"
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/tls"
	"crypto/x509"
	"crypto/x509/pkix"
	"errors"
	"fmt"
	"io"
	"math/big"
	"mime"
	"net"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"path"
	"path/filepath"
	"strings"
	"time"
)

// Config sets up the front.
type Config struct {
	// Listen is the TLS address, such as ":8443".
	Listen string
	// Upstream is the t3 server's HTTP address, such as "127.0.0.1:3780".
	Upstream string
	// NoDeflate removes the client's WebSocket compression offer, so the
	// connection runs uncompressed like the unchanged iOS app's.
	NoDeflate bool
	// ClientDir is a web client build to serve; empty forwards everything.
	ClientDir string
}

// Handler serves the client build's files and page loads from ClientDir and
// forwards every other request to the upstream server.
func Handler(cfg Config) http.Handler {
	proxy := proxyHandler(cfg)
	if cfg.ClientDir == "" {
		return proxy
	}
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if file, ok := clientFile(cfg.ClientDir, r); ok {
			serveClientFile(w, r, file)
			return
		}
		proxy.ServeHTTP(w, r)
	})
}

// clientFile is the build's file for a request: a file the build has, or
// index.html for a page load, as the client routes its pages itself (/pair,
// a thread). Everything else, API calls and the WebSocket, goes to t3.
func clientFile(dir string, r *http.Request) (string, bool) {
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		return "", false
	}
	file := filepath.Join(dir, filepath.FromSlash(path.Clean("/"+r.URL.Path)))
	if info, err := os.Stat(file); err == nil && info.Mode().IsRegular() {
		return file, true
	}
	if r.Header.Get("Sec-Fetch-Mode") == "navigate" {
		return filepath.Join(dir, "index.html"), true
	}
	return "", false
}

// serveClientFile answers as t3 does for its own static files: hashed assets
// cached for good, the rest revalidated, text gzipped when the browser takes
// it. The bytes on the lab's link stay what they were when t3 served the
// build.
func serveClientFile(w http.ResponseWriter, r *http.Request, file string) {
	body, err := os.ReadFile(file)
	if err != nil {
		fmt.Printf("front: %v\n", err)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}
	h := w.Header()
	ctype := mime.TypeByExtension(filepath.Ext(file))
	if ctype == "" {
		ctype = http.DetectContentType(body)
	}
	h.Set("Content-Type", ctype)
	if strings.HasPrefix(r.URL.Path, "/assets/") && filepath.Base(file) != "index.html" {
		h.Set("Cache-Control", "public, max-age=31536000, immutable")
	} else {
		h.Set("Cache-Control", "no-cache")
	}
	var out io.Writer = w
	var gz *gzip.Writer
	if compressible(ctype) && strings.Contains(r.Header.Get("Accept-Encoding"), "gzip") {
		h.Set("Content-Encoding", "gzip")
		h.Add("Vary", "Accept-Encoding")
		gz = gzip.NewWriter(w)
		out = gz
	} else {
		h.Set("Content-Length", fmt.Sprint(len(body)))
	}
	if r.Method == http.MethodHead {
		return
	}
	_, err = out.Write(body)
	if gz != nil {
		err = errors.Join(err, gz.Close())
	}
	if err != nil {
		fmt.Printf("front: write %s: %v\n", r.URL.Path, err)
	}
}

// compressible says which types t3's HTTP compression compresses.
func compressible(ctype string) bool {
	t, _, _ := strings.Cut(ctype, ";")
	t = strings.ToLower(strings.TrimSpace(t))
	switch t {
	case "application/json", "application/javascript", "application/xml", "image/svg+xml", "application/wasm":
		return true
	}
	return strings.HasPrefix(t, "text/") || strings.HasSuffix(t, "+json") || strings.HasSuffix(t, "+xml")
}

// proxyHandler forwards requests to the upstream server.
func proxyHandler(cfg Config) http.Handler {
	target := &url.URL{Scheme: "http", Host: cfg.Upstream}
	return &httputil.ReverseProxy{
		Rewrite: func(r *httputil.ProxyRequest) {
			r.SetURL(target)
			// The server checks the browser's Origin against the Host it sees.
			r.Out.Host = r.In.Host
			r.SetXForwarded()
			if cfg.NoDeflate {
				r.Out.Header.Del("Sec-WebSocket-Extensions")
			}
		},
		// Streams such as server-sent events must not wait in a buffer.
		FlushInterval: -1,
	}
}

// Serve runs the front until ctx ends.
func Serve(ctx context.Context, cfg Config) error {
	cert, err := selfSigned()
	if err != nil {
		return err
	}
	srv := &http.Server{
		Handler:           Handler(cfg),
		TLSConfig:         &tls.Config{Certificates: []tls.Certificate{cert}},
		ReadHeaderTimeout: time.Minute,
	}
	ln, err := net.Listen("tcp", cfg.Listen)
	if err != nil {
		return err
	}
	go func() {
		<-ctx.Done()
		if err := srv.Close(); err != nil {
			fmt.Printf("front: close: %v\n", err)
		}
	}()
	if err := srv.ServeTLS(ln, "", ""); !errors.Is(err, http.ErrServerClosed) {
		return err
	}
	return nil
}

func selfSigned() (tls.Certificate, error) {
	key, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		return tls.Certificate{}, err
	}
	tmpl := &x509.Certificate{
		SerialNumber: big.NewInt(1),
		Subject:      pkix.Name{CommonName: "t3lab"},
		NotBefore:    time.Now().Add(-time.Hour),
		NotAfter:     time.Now().Add(24 * time.Hour),
		KeyUsage:     x509.KeyUsageDigitalSignature,
		ExtKeyUsage:  []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		IPAddresses:  []net.IP{net.ParseIP("10.231.0.1"), net.ParseIP("10.231.1.1")},
	}
	der, err := x509.CreateCertificate(rand.Reader, tmpl, tmpl, &key.PublicKey, key)
	if err != nil {
		return tls.Certificate{}, err
	}
	return tls.Certificate{Certificate: [][]byte{der}, PrivateKey: key}, nil
}
