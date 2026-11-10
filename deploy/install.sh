#!/usr/bin/env bash
# Installs t3opt on a Linux host (arm64 or x86_64) that is on a tailnet, as
# two user services beside anything already there:
#
# - t3opt-server: the stock t3 v0.1.7 release, data in ~/.t3opt, localhost
#   port 13773.
# - t3opt-sidecar: serves the home-screen web app on localhost port 12480 and
#   syncs to phones over WebRTC on UDP 12444 of the tailnet address.
#
# and one `tailscale serve` route, https port 12443 to the sidecar. Without
# --main it never touches ~/.t3 or other tailscale serve routes. Running it
# again upgrades in place. See deploy/README.md.
#
# With --main, also:
#
# - t3opt-sidecar-main: a second sidecar for the host's own t3code
#   (127.0.0.1:3773, ~/.t3), on localhost port 12481 and UDP 12445, with
#   one more route, https port 14443 to it. It is a client of that t3code:
#   the only thing it adds to ~/.t3 is its own login session, from
#   `t3 pair --base-dir ~/.t3`.
#
# Usage: install.sh [--main] [--assets <dir>]
#   Takes the t3opt files from the GitHub release named by $T3OPT_RELEASE
#   (a tag such as v0.2.0, default the latest release), or from <dir>, which
#   holds the same files as a release.
set -euo pipefail

repo=djosh34/t3-optimizations
t3_version=0.1.7
share="$HOME/.local/share/t3opt"
# The unit files read %h/.config/t3opt/env, so this is not $XDG_CONFIG_HOME.
config="$HOME/.config"
units="$config/systemd/user"

assets=""
main=no
while [ $# -gt 0 ]; do
  case "$1" in
    --main) main=yes; shift ;;
    --assets) assets="$(realpath "${2:?--assets needs a directory}")"; shift 2 ;;
    *) echo "usage: install.sh [--main] [--assets <dir>]" >&2; exit 2 ;;
  esac
done
services=(t3opt-server.service t3opt-sidecar.service)
[ "$main" = no ] || services+=(t3opt-sidecar-main.service)

say() { echo "install: $*" >&2; }
die() { say "$*"; exit 1; }

for tool in curl tar sha256sum systemctl tailscale; do
  command -v "$tool" >/dev/null || die "$tool is missing"
done

case "$(uname -m)" in
  aarch64 | arm64) arch=arm64 t3_arch=arm64 ;;
  x86_64) arch=x86_64 t3_arch=x64 ;;
  *) die "no build for $(uname -m)" ;;
esac

tailnet_ip="$(tailscale ip -4 | head -n 1)"
[ -n "$tailnet_ip" ] || die "this host has no tailnet IPv4 address"
# The first DNSName in the status is this host's own.
host="$(tailscale status --json | sed -n 's/^ *"DNSName": "\(.*\)\.",$/\1/p' | head -n 1)"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# Fetches each named file of the t3opt release into $tmp and checks it
# against the release's SHA256SUMS.
fetch() {
  local release="${T3OPT_RELEASE:-latest}" base name
  if [ "$release" = latest ]; then
    base="https://github.com/$repo/releases/latest/download"
  else
    base="https://github.com/$repo/releases/download/$release"
  fi
  for name in SHA256SUMS "$@"; do
    if [ -n "$assets" ]; then
      cp "$assets/$name" "$tmp/$name"
    else
      curl -fsSL --retry 3 -o "$tmp/$name" "$base/$name"
    fi
  done
  for name in "$@"; do
    grep -E "^[0-9a-f]{64}  $name\$" "$tmp/SHA256SUMS" >"$tmp/sum" || die "$name is not in SHA256SUMS"
    (cd "$tmp" && sha256sum --check --quiet sum) || die "$name does not match SHA256SUMS"
  done
}

# 1. The stock t3 release, in a folder of its own.
t3="$share/t3-$t3_version/t3"
if [ -x "$t3" ] && "$t3" --version >/dev/null 2>&1; then
  say "t3 $t3_version is already in $share"
else
  say "downloading t3 $t3_version"
  archive="t3-$t3_version-linux-$t3_arch.tar.gz"
  base="https://github.com/pingdotgg/t3code/releases/download/v$t3_version"
  curl -fsSL --retry 3 -o "$tmp/t3-sums" "$base/SHA256SUMS"
  curl -fsSL --retry 3 -o "$tmp/$archive" "$base/$archive"
  grep -E "^[0-9a-f]{64}  \*?$archive\$" "$tmp/t3-sums" | sed 's/  \*/  /' >"$tmp/sum" ||
    die "$archive is not in t3's SHA256SUMS"
  (cd "$tmp" && sha256sum --check --quiet sum) || die "$archive does not match t3's SHA256SUMS"
  mkdir -p "$share"
  rm -rf "$share/t3-$t3_version.new"
  mkdir "$share/t3-$t3_version.new"
  tar -xzf "$tmp/$archive" -C "$share/t3-$t3_version.new" --strip-components=1
  "$share/t3-$t3_version.new/t3" --version >/dev/null || die "the downloaded t3 does not run"
  rm -rf "$share/t3-$t3_version"
  mv "$share/t3-$t3_version.new" "$share/t3-$t3_version"
fi

# 2. The sidecar, the web app and the unit files.
say "fetching the t3opt files"
fetch "t3opt-sidecar-linux-$arch" t3opt-web.tar.gz "${services[@]}"
mkdir -p "$share" "$units" "$config/t3opt"
install -m 755 "$tmp/t3opt-sidecar-linux-$arch" "$share/sidecar.new"
mv "$share/sidecar.new" "$share/sidecar"
rm -rf "$share/web.new"
mkdir "$share/web.new"
tar -xzf "$tmp/t3opt-web.tar.gz" -C "$share/web.new"
rm -rf "$share/web"
mv "$share/web.new" "$share/web"
for unit in "${services[@]}"; do
  install -m 644 "$tmp/$unit" "$units/"
done

# The services get this PATH, so t3 finds the agents' CLIs as this shell does.
cat >"$config/t3opt/env" <<EOF
PATH=$PATH
T3OPT_TAILNET_IP=$tailnet_ip
EOF

# 3. Start them, now and after every boot.
if [ "$(loginctl show-user "$USER" -p Linger --value 2>/dev/null)" != yes ]; then
  say "turning on lingering, so the services run without a login"
  loginctl enable-linger "$USER" ||
    die "could not; run: sudo loginctl enable-linger $USER, then this script again"
fi
systemctl --user daemon-reload
systemctl --user enable "${services[@]}"
systemctl --user restart "${services[@]}"

# 4. HTTPS on the tailnet, one route per sidecar. Leaves every other route
# as it is.
# Usage: serve <https port> <sidecar's local port> <unit> <t3 base dir>
serve() {
  tailscale serve --bg --https="$1" "http://127.0.0.1:$2" >/dev/null ||
    die "tailscale serve failed; this user may need to be the operator: sudo tailscale set --operator=$USER"
  say "waiting for $3"
  for _ in $(seq 60); do
    if curl -fsS -o /dev/null "http://127.0.0.1:$2/" 2>/dev/null; then
      say "$3: https://${host:-<this host>}:$1"
      say "pair a phone with a code from: $t3 pair --base-dir $4"
      return
    fi
    sleep 2
  done
  die "$3 did not answer in 2 minutes; see journalctl --user -u $3"
}
serve 12443 12480 t3opt-sidecar "$HOME/.t3opt"
[ "$main" = no ] || serve 14443 12481 t3opt-sidecar-main "$HOME/.t3"
say "all services are up"
