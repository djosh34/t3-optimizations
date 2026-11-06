#!/usr/bin/env bash
# Runs the tests of every t3code workspace package with files changed
# against a base ref (default origin/main). Uncommitted and untracked files
# count. Usage: scripts/t3code-changed-tests.sh [base-ref]
set -euo pipefail

base_ref="${1:-origin/main}"
cd "$(git rev-parse --show-toplevel)"

base="$(git merge-base "$base_ref" HEAD)"
changed="$(
  # Without renames, a moved file counts for both its old and new package.
  git diff --no-renames --name-only "$base" -- t3code
  git ls-files --others --exclude-standard -- t3code
)"

# The workspace package globs of t3code/pnpm-workspace.yaml.
packages="$(
  printf '%s\n' "$changed" |
    sed -nE \
      -e 's#^t3code/((apps|packages|infra)/[^/]+)/.*#\1#p' \
      -e 's#^t3code/(evals|oxlint-plugin-t3code|scripts)/.*#\1#p' |
    sort -u
)"

cd t3code
# Use the vp of t3code's own install: the global one runs its own vitest.
export PATH="$PWD/node_modules/.bin:$PATH"
ran=0
for pkg in $packages; do
  if [[ ! -f "$pkg/package.json" ]]; then
    continue
  fi
  if [[ "$(node -p "Boolean(require('./$pkg/package.json').scripts?.test)")" != true ]]; then
    echo "t3code/$pkg: no test script"
    continue
  fi
  echo "t3code/$pkg: running tests"
  # Run as CI does: no T3CODE_* settings from this shell (T3CODE_HOME may name
  # the official ~/.t3) and no global git config (URL rewrites break tests).
  env $(compgen -e | grep '^T3CODE_' | sed 's/^/-u /') GIT_CONFIG_GLOBAL=/dev/null \
    vp run --filter "./$pkg" test
  ran=$((ran + 1))
done
echo "t3code: ran the tests of $ran changed package(s) against $base_ref"
