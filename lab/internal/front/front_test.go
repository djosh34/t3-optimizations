package front

import (
	"io"
	"net/http"
	"net/http/httptest"
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
