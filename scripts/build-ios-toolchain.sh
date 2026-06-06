#!/usr/bin/env bash
#
# Install the cargo-makepad iOS toolchain (Rust target + Xcode probe) used by
# build-ios.sh. Requires Xcode + iOS Simulator runtime to be already installed
# from the App Store.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if ! command -v cargo-makepad >/dev/null 2>&1; then
  echo "error: 'cargo-makepad' not found in PATH" >&2
  echo "       install with: cargo install --path /Users/x/code/makepad/tools/cargo_makepad" >&2
  exit 1
fi

if ! xcode-select -p >/dev/null 2>&1; then
  echo "error: Xcode command-line tools not found." >&2
  echo "       install Xcode from the App Store, then run: xcode-select --install" >&2
  exit 1
fi

echo "==> Installing iOS toolchain"
(unset CARGO_TARGET_DIR; cargo makepad apple ios install-toolchain)

echo
echo "Done. Run ./scripts/build-ios.sh to build the simulator .app."
