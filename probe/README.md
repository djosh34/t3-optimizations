# iOS transport probe (#22)

Throwaway probe for [#22](https://github.com/djosh34/t3-optimizations/issues/22).
One Go binary serves a page over the tailnet that runs every test by itself and
posts the result back; the server posts each result as a comment on #22.

- TCP 6443: HTTPS/1.1 (page, service worker, WebRTC signaling, results, WebSocket)
- UDP 6443: HTTP/3 (same routes, WebTransport at `/wt`, WebSocket over H3 per RFC 9220 at `/ws`), advertised via `Alt-Svc`
- UDP 6444: WebRTC ICE (pion, ICE-lite, tailscale0 addresses only, no STUN/TURN)

`third_party/webtransport-go` is webtransport-go v0.13.0 with one change: it
accepts sessions from clients that do not announce HTTP/3 datagrams, so iOS
streams can be tested even if WebKit bug 322201 applies.

## Run

```sh
D=~/.local/share/t3probe; mkdir -p $D/results
sudo tailscale cert --cert-file $D/cert.pem --key-file $D/key.pem lima-opencode-vm.tail2fc05a.ts.net
sudo chown $USER: $D/*.pem
go build -o $D/t3probe .
systemd-run --user --unit=t3-probe --working-directory=$D -p Restart=on-failure \
  -E PATH=/usr/bin:/usr/local/bin $D/t3probe -cert cert.pem -key key.pem -out results
```

Open `https://lima-opencode-vm.tail2fc05a.ts.net:6443/`. Query options:
`?dry=1` (save only, no GitHub comment), `?label=text`, `?net=Wi-Fi|cellular`.

Stop: `systemctl --user stop t3-probe`. Raw results: `~/.local/share/t3probe/results/`.

`go run ./cmd/h3wscheck` checks WebSocket over HTTP/3 against the server.
`t3probe -render file.json` prints the comment a saved result would produce.
