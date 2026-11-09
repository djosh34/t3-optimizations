# Web app end to end

Commit 982979da9a8f, seed 1, outage 20s. WebKit (Playwright 1.64.1) at the iPhone 17 Pro's size in the lab's phone container; the sidecar and the stock t3 server in the server namespace, the lab front for HTTPS. Between steps that shape it, the link is profile A.

| Step | Result |
|---|---|
| pairing | ok in 83.3 s |
| service-worker | ok in 0.1 s: version 9d2f8850b186dc2b |
| add-project-and-thread | ok in 14.8 s: thread 443ae7a2-ea71-4feb-8ce7-8599433d1f2e |
| stream-and-approval | ok in 41.3 s |
| interrupt | ok in 6.6 s |
| upload | ok in 11.1 s |
| prompt-on-case70 | ok in 49.4 s: queued > confirmed in 5.7 s |
| thread-fills-newest-first | ok in 43.8 s: newest before oldest |
| reload-old-connection-alive | ok in 6.2 s: generation 3 to 4 |
| outage-recovered-on-the-same-connection | ok in 26.2 s: 2 ICE restarts, gap 24.0 s |
| history-and-images-without-http | ok in 4.2 s |
| offline-open | ok in 3.1 s |
| websocket-fallback-mid-session | ok in 207.8 s: generation 6, queued > confirmed in 5.0 s |
| websocket-fallback | ok in 35.5 s: confirmed in 12.9 s |
| update | ok in 29.6 s: 9d2f8850b186dc2b to 9d31bc40e851cef1 |
| deleted-while-sidecar-down | ok in 69.1 s |
| cache-lost-token-kept | ok in 9.4 s |
| token-lost | ok in 17.8 s |

The app posted 18 link reports during the test; the sidecar logged them in `sidecar/link-reports.jsonl`.

Screenshots are in `phone/`: each name once as the Safari tab (`-safari`) and once as the home-screen app (`-home-screen`).
