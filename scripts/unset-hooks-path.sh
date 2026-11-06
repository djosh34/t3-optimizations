#!/usr/bin/env bash
# Removes core.hooksPath, which `vp install` in t3code/ sets for the whole
# repo and every worktree. A setting that is already absent is fine.
set -uo pipefail

git config --unset-all core.hooksPath
status=$?
# With --unset-all, git config exits 5 only when the key is not set.
if [[ $status -ne 0 && $status -ne 5 ]]; then
  echo "could not unset core.hooksPath (git config exit $status)" >&2
  exit "$status"
fi
