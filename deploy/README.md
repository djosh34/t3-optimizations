# deploy

How t3opt runs on a host, beside an official t3code that it does not change ([issue #48](https://github.com/djosh34/t3-optimizations/issues/48); the main sidecar, [issue #85](https://github.com/djosh34/t3-optimizations/issues/85), is its client).

| What | Where |
|---|---|
| `t3opt-server.service` | the stock t3 v0.1.7 release, `t3 serve --base-dir ~/.t3opt` on 127.0.0.1:13773 |
| `t3opt-sidecar.service` | the sidecar: the web app on 127.0.0.1:12480, WebRTC on UDP 12444 of the tailnet address |
| `tailscale serve` route | `https://<host>:12443` to the sidecar |
| `t3opt-sidecar-main.service` | only with `install.sh --main`: a second sidecar for the host's own t3code (127.0.0.1:3773, `~/.t3`), the web app on 127.0.0.1:12481, WebRTC on UDP 12445, `MemoryMax=768M` |
| its `tailscale serve` route | `https://<host>:14443` to that sidecar |
| Programs | `~/.local/share/t3opt/` (`t3-0.1.7/`, `sidecar`, `web/`) |
| Settings | `~/.config/t3opt/env`: the tailnet address and the `PATH` the agents' CLIs are found on |
| Data | `~/.t3opt` (t3's), `~/.t3opt-sidecar` (the sidecar's), `~/.t3opt-sidecar-main` (the main sidecar's) |

All are user services, enabled, restarted on failure, and running after a reboot when lingering is on (`loginctl enable-linger`). `t3opt-server` does not read `~/.t3/service.env`; the agents log in with their own files in the home directory, as they do for any t3. The sidecar pairs itself with `t3opt-server` (`t3 pair --base-dir ~/.t3opt`).

## Install

On a Linux host (arm64 or x86_64) on a tailnet, as the user who runs the services, with that user as tailscale's operator (`sudo tailscale set --operator=$USER`):

```sh
curl -fsSLO https://github.com/djosh34/t3-optimizations/releases/latest/download/install.sh
bash install.sh
```

It downloads t3 v0.1.7 from [t3code's release](https://github.com/pingdotgg/t3code/releases/tag/v0.1.7) and the sidecar and web app from this repo's release, checks them against each release's `SHA256SUMS`, writes and starts both services, and adds the 12443 route. `T3OPT_RELEASE=v0.2.0 bash install.sh` picks a release; `install.sh --assets <dir>` takes the files from a folder instead (the `release` artifact of a run of `.github/workflows/release.yml`). Running it again upgrades in place.

`bash install.sh --main` also installs and starts `t3opt-sidecar-main` and adds the 14443 route. That sidecar is a client of the t3code already on the host (`~/.t3`, port 3773). The only thing it adds to `~/.t3` is its own login session: it pairs itself with `t3 pair --base-dir ~/.t3`, loads a thread when a phone opens it, and keeps at most 256 MB of threads and images (`--cache-mb`). Run the upgrade with `--main` too, or the main sidecar keeps running the old binary.

## Pair a phone

Mint a pairing code. It works once, for 5 minutes:

```sh
~/.local/share/t3opt/t3-0.1.7/t3 pair --base-dir ~/.t3opt
```

Open `https://<host>:12443` on the phone and enter the line after `Token:`. In Safari, Share, then Add to Home Screen, gives the home-screen app.

For the main sidecar, the code comes from the host's own t3code, and the phone opens `https://<host>:14443`:

```sh
~/.local/share/t3opt/t3-0.1.7/t3 pair --base-dir ~/.t3
```

## Stop or remove

```sh
systemctl --user disable --now t3opt-sidecar-main           # the main sidecar only
tailscale serve --https=14443 off                           # and its route
systemctl --user disable --now t3opt-sidecar t3opt-server   # stop, and not after a reboot
tailscale serve --https=12443 off                           # remove the route
rm -rf ~/.local/share/t3opt ~/.config/t3opt ~/.config/systemd/user/t3opt-*.service
rm -rf ~/.t3opt ~/.t3opt-sidecar ~/.t3opt-sidecar-main      # the data: projects, threads, phones
```

Removing the main sidecar leaves `~/.t3` as it is; its login sessions are revoked in t3's connections list.

## Live check

`deploy/live-check.sh https://<host>:12443 <out dir>` runs `live-check.mjs` in Playwright WebKit (the lab's phone image) at the iPhone 17 Pro's size, as the Safari tab and as the home-screen app: it pairs with a fresh code, adds a project, starts a thread, sends a short prompt to the default agent and waits for the reply, with a screenshot of each step in the out dir.

`deploy/main-live-check.sh https://<host>:14443 <out dir>` checks the main sidecar without touching the owner's threads' read marks. `crates/native-phone`'s `open-threads` opens the newest, the oldest archived, a delegated-task and the largest thread with `sidecar.focus` and times each; then `main-live-check.mjs` in WebKit starts one thread, `t3opt live check`, in the `t3-optimizations` project with GPT-6 Luna, sends "Reply with the words: t3opt live check ok" and times the reply as the Safari tab, then opens that thread as the home-screen app. It samples both services' memory every 5 s. Its output names no thread; the out dir holds thread ids and screenshots of the t3code, for a local look only.

## Release

`.github/workflows/release.yml` builds static (musl) sidecars for linux x86_64 and arm64, the built web app, these unit files and `install.sh`, with `SHA256SUMS`. On a tag `v*` it publishes them as a GitHub release with the notes in `release-notes/<tag>.md`. On a pull request that changes `deploy/` or the workflow, or started by hand, it only builds them, as the `release` artifact. Started by hand with `tag`, it publishes an existing tag whose first run failed.
