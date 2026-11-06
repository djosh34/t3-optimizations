// Package front is the lab's HTTPS origin. It terminates TLS with a fresh
// self-signed certificate and forwards everything to the t3 server, like
// `tailscale serve` does in production. HTTPS makes the page a secure
// context, so service workers work in the lab.
package front

import (
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/tls"
	"crypto/x509"
	"crypto/x509/pkix"
	"errors"
	"fmt"
	"math/big"
	"net"
	"net/http"
	"net/http/httputil"
	"net/url"
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
}

// Handler forwards requests to the upstream server.
func Handler(cfg Config) http.Handler {
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
