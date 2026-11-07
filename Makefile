# Local checks to run before every push. CI runs these too, plus the full
# t3code typecheck and test suites, which need more memory than the dev VM
# has free.

# The t3code tests that run are those of packages changed against this ref.
BASE ?= origin/main

# t3code tasks must use the vp of t3code's own install, not the global one.
export PATH := $(CURDIR)/t3code/node_modules/.bin:$(PATH)

.PHONY: check rust go t3code setup

check: rust go t3code

rust:
	cd crates && cargo fmt --check
	cd crates && cargo clippy --workspace --all-targets -- -D warnings
	cd crates && cargo test --workspace
	cd crates && cargo test --manifest-path vendor/sctp-proto/Cargo.toml
	cd crates && cargo test --manifest-path vendor/is/Cargo.toml
	cd crates && cargo build -p sync-core --target wasm32-unknown-unknown

go:
	@files="$$(gofmt -l lab)"; if [ -n "$$files" ]; then echo "gofmt needed:"; echo "$$files"; exit 1; fi
	cd lab && go vet ./...
	cd lab && go test ./...

t3code:
	cd t3code && vp check
	scripts/t3code-changed-tests.sh $(BASE)

# Installs the t3code dependencies, then removes the git hooks that the
# install registers for the whole repo.
setup:
	cd t3code && vp install
	scripts/unset-hooks-path.sh
