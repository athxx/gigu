#!/usr/bin/env bash
#
# Run gigu locally on macOS host.
#
# Neutralises the user's global ~/.cargo/config.toml overrides:
#   - target-dir = /tmp/cargo  → restore workspace target/
#   - rustc-wrapper = sccache  → off (not needed)
#   - cfg(target_os="macos") rustflags = -fuse-ld=lld  → host clang has no lld
# The project .cargo/config.toml tries to clear these but the cfg-form rustflags
# leak through anyway; clearing RUSTFLAGS at invocation is the reliable fix.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

unset CARGO_TARGET_DIR
export CARGO_BUILD_RUSTC_WRAPPER=""
export RUSTFLAGS=""

cargo run --bin gigu --target-dir="$ROOT_DIR/target" "$@"
