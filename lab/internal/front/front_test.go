package front

import (
	"compress/gzip"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestHandler(t *testing.T) {
	upstream := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if _, err := io.WriteString(w, r.Host+" "+r.Header.Get("Sec-WebSocket-Extensions")); err != nil {
			t.Error(err)
		}
	}))
	defer upstream.Close()
	addr := strings.TrimPrefix(upstream.URL, "http://")
	for _, c := range []struct {
		noDeflate bool
		want      string
	}{
		{false, "10.231.0.1:8443 permessage-deflate"},
		{true, "10.231.0.1:8443 "},
	} {
		req := httptest.NewRequest("GET", "https://10.231.0.1:8443/ws", nil)
		req.Header.Set("Sec-WebSocket-Extensions", "permessage-deflate")
		rec := httptest.NewRecorder()
		Handler(Config{Upstream: addr, NoDeflate: c.noDeflate}).ServeHTTP(rec, req)
		if got := rec.Body.String(); got != c.want {
			t.Errorf("noDeflate=%v: upstream saw %q, want %q", c.noDeflate, got, c.want)
		}
	}
}

func TestClientDir(t *testing.T) {
	upstream := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if _, err := io.WriteString(w, "t3 "+r.URL.Path); err != nil {
			t.Error(err)
		}
	}))
	defer upstream.Close()
	dir := t.TempDir()
	for name, body := range map[string]string{"index.html": "<p>app</p>", "assets/app-1234abcd.js": "let x = 1;", "favicon.ico": "icon"} {
		file := filepath.Join(dir, filepath.FromSlash(name))
		if err := os.MkdirAll(filepath.Dir(file), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(file, []byte(body), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	h := Handler(Config{Upstream: strings.TrimPrefix(upstream.URL, "http://"), ClientDir: dir})
	for _, c := range []struct {
		path     string
		navigate bool
		want     string
	}{
		{"/", true, "<p>app</p>"},
		{"/pair", true, "<p>app</p>"},
		{"/assets/app-1234abcd.js", false, "let x = 1;"},
		{"/favicon.ico", false, "icon"},
		{"/../index.html", false, "<p>app</p>"},
		{"/api/auth/session", false, "t3 /api/auth/session"},
		{"/ws", false, "t3 /ws"},
	} {
		req := httptest.NewRequest("GET", "https://10.231.0.1:8443"+c.path, nil)
		if c.navigate {
			req.Header.Set("Sec-Fetch-Mode", "navigate")
		}
		rec := httptest.NewRecorder()
		h.ServeHTTP(rec, req)
		if got := rec.Body.String(); got != c.want {
			t.Errorf("%s: got %q, want %q", c.path, got, c.want)
		}
	}

	// Text is gzipped for a browser that takes it; hashed assets are kept.
	req := httptest.NewRequest("GET", "https://10.231.0.1:8443/assets/app-1234abcd.js", nil)
	req.Header.Set("Accept-Encoding", "gzip, deflate, br")
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, req)
	res := rec.Result()
	if res.Header.Get("Content-Encoding") != "gzip" || !strings.Contains(res.Header.Get("Cache-Control"), "immutable") {
		t.Fatalf("headers %v", res.Header)
	}
	zr, err := gzip.NewReader(res.Body)
	if err != nil {
		t.Fatal(err)
	}
	if body, err := io.ReadAll(zr); err != nil || string(body) != "let x = 1;" {
		t.Errorf("gzipped body %q, %v", body, err)
	}
}
