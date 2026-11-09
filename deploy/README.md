# deploy

How t3opt runs on a host, beside an official t3code that it never touches ([issue #48](https://github.com/djosh34/t3-optimizations/issues/48)).

| What | Where |
|---|---|
| `t3opt-server.service` | the stock t3 v0.1.7 release, `t3 serve --base-dir ~/.t3opt` on 127.0.0.1:13773 |
| `t3opt-sidecar.service` | the sidecar: the web app on 127.0.0.1:12480, WebRTC on UDP 12444 of the tailnet address |
| `tailscale serve` route | `https://<host>:12443` to the sidecar |
| Programs | `~/.local/share/t3opt/` (`t3-0.1.7/`, `sidecar`, `web/`) |
| Settings | `~/.config/t3opt/env`: the tailnet address and the `PATH` the agents' CLIs are found on |
| Data | `~/.t3opt` (t3's), `~/.t3opt-sidecar` (the sidecar's) |

Both are user services, enabled, restarted on failure, and running after a reboot when lingering is on (`loginctl enable-linger`). `t3opt-server` does not read `~/.t3/service.env`; the agents log in with their own files in the home directory, as they do for any t3. The sidecar pairs itself with `t3opt-server` (`t3 pair --base-dir ~/.t3opt`).

## Install

On a Linux host (arm64 or x86_64) on a tailnet, as the user who runs the services, with that user as tailscale's operator (`sudo tailscale set --operator=$USER`):

```sh
curl -fsSLO https://github.com/djosh34/t3-optimizations/releases/latest/download/install.sh
bash install.sh
```

It downloads t3 v0.1.7 from [t3code's release](https://github.com/pingdotgg/t3code/releases/tag/v0.1.7) and the sidecar and web app from this repo's release, checks them against each release's `SHA256SUMS`, writes and starts both services, and adds the 12443 route. `T3OPT_RELEASE=v0.1.0 bash install.sh` picks a release; `install.sh --assets <dir>` takes the files from a folder instead (the `release` artifact of a run of `.github/workflows/release.yml`). Running it again upgrades in place.

## Pair a phone

Mint a pairing code. It works once, for 5 minutes:

```sh
~/.local/share/t3opt/t3-0.1.7/t3 pair --base-dir ~/.t3opt
```

Open `https://<host>:12443` on the phone and enter the line after `Token:`. In Safari, Share, then Add to Home Screen, gives the home-screen app.

## Stop or remove

```sh
systemctl --user disable --now t3opt-sidecar t3opt-server   # stop, and not after a reboot
tailscale serve --https=12443 off                           # remove the route
rm -rf ~/.local/share/t3opt ~/.config/t3opt ~/.config/systemd/user/t3opt-*.service
rm -rf ~/.t3opt ~/.t3opt-sidecar                             # the data: projects, threads, phones
```

## Live check

`deploy/live-check.sh https://<host>:12443 <out dir>` runs `live-check.mjs` in Playwright WebKit (the lab's phone image) at the iPhone 17 Pro's size, as the Safari tab and as the home-screen app: it pairs with a fresh code, adds a project, starts a thread, sends a short prompt to the default agent and waits for the reply, with a screenshot of each step in the out dir.

## Release

`.github/workflows/release.yml` builds static (musl) sidecars for linux x86_64 and arm64, the built web app, these unit files and `install.sh`, with `SHA256SUMS`. On a tag `v*` it publishes them as a GitHub release with the notes in `release-notes/<tag>.md`. On a pull request that changes `deploy/` or the workflow, or started by hand, it only builds them, as the `release` artifact. Started by hand with `tag`, it publishes an existing tag whose first run failed.
