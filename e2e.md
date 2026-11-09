# Web app end to end

Commit b3ce06cc8876-dirty, seed 1, outage 20s. WebKit (Playwright 1.64.1) at the iPhone 17 Pro's size in the lab's phone container; the sidecar and the stock t3 server in the server namespace, the lab front for HTTPS. Between steps that shape it, the link is profile A.

| Step | Result |
|---|---|
| pairing | ok in 80.8 s |
| service-worker | ok in 0.1 s: version 7556aa26a3e3beb1 |
| add-project-and-thread | ok in 23.0 s: thread b4fae30d-1b6b-4b78-9263-f7b92c6cb6a6 |
| stream-and-approval | ok in 41.5 s |
| interrupt | ok in 4.7 s |
| upload | ok in 8.8 s |
| prompt-on-case70 | ok in 55.4 s: queued > sent > confirmed in 7.9 s |
| thread-fills-newest-first | ok in 40.8 s: newest before oldest |
| reload-old-connection-alive | ok in 6.3 s: generation 3 to 4 |
| outage-recovered-on-the-same-connection | ok in 26.2 s: 2 ICE restarts, gap 24.3 s |
| history-and-images-without-http | ok in 3.6 s |
| offline-open | ok in 3.1 s |
| websocket-fallback-mid-session | ok in 218.2 s: generation 6, sent > confirmed in 15.0 s |
| websocket-fallback | ok in 29.5 s: queued > confirmed in 5.3 s |
| update | ok in 32.8 s: 7556aa26a3e3beb1 to 5ffbd03c405916e8 |
| deleted-while-sidecar-down | ok in 68.4 s |
| cache-lost-token-kept | ok in 11.6 s |
| token-lost | ok in 22.8 s |

The app posted 17 link reports during the test; the sidecar logged them in `sidecar/link-reports.jsonl`.

Screenshots are in `phone/`: each name once as the Safari tab (`-safari`) and once as the home-screen app (`-home-screen`).
