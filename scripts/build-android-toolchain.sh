#!/usr/bin/env bash
#
# Install the cargo-makepad Android toolchain (NDK + SDK + rust target) used
# by build-android.sh.
#
# Env overrides (all optional):
#   ANDROID_ABI           Android ABI (default: aarch64)
#   MAKEPAD_ANDROID_SDK   Where the SDK lives (default: ./android_33_sdk).
#                         cargo-makepad downloads/extracts everything underneath.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ABI="${ANDROID_ABI:-aarch64}"
SDK_PATH="${MAKEPAD_ANDROID_SDK:-$ROOT_DIR/android_33_sdk}"

if ! command -v cargo-makepad >/dev/null 2>&1; then
  echo "error: 'cargo-makepad' not found in PATH" >&2
  echo "       install with: cargo install --path /Users/x/code/makepad/tools/cargo_makepad" >&2
  exit 1
fi

echo "==> Installing Android toolchain (abi=$ABI, sdk=$SDK_PATH)"
mkdir -p "$SDK_PATH"

(unset CARGO_TARGET_DIR; cargo makepad android \
  --abi="$ABI" \
  --sdk-path="$SDK_PATH" \
  install-toolchain)

echo
echo "Done. Android SDK at: $SDK_PATH"
echo "Run ./scripts/build-android.sh to build the APK."
