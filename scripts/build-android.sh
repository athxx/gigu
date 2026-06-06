#!/usr/bin/env bash
#
# Build a release APK for gigu via cargo-makepad.
#
# Env overrides (all optional):
#   BIN_NAME              cargo bin/package to build (default: gigu)
#   ANDROID_PACKAGE_NAME  Android package name      (default: dev.gigu.gigu)
#   ANDROID_APP_LABEL     Android app label         (default: gigu)
#   ANDROID_ABI           Android ABI               (default: aarch64)
#   MAKEPAD_ANDROID_SDK   Path to android_33_sdk    (default: ./android_33_sdk)
#
# Output:
#   dist/android/<bin>-<abi>.apk

set -euo pipefail

MAKEPAD_ANDROID_SDK="/Users/x/code/makepad/tools/cargo_makepad/android_33_macos_aarch64"


ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="${BIN_NAME:-gigu}"
PKG="${ANDROID_PACKAGE_NAME:-dev.gigu.gigu}"
LABEL="${ANDROID_APP_LABEL:-gigu}"
ABI="${ANDROID_ABI:-aarch64}"
SDK_PATH="${MAKEPAD_ANDROID_SDK:-$ROOT_DIR/android_33_sdk}"
OUT_DIR="$ROOT_DIR/dist/android"
FINAL_APK="$OUT_DIR/${BIN_NAME}-${ABI}.apk"

if ! command -v cargo-makepad >/dev/null 2>&1; then
  echo "error: 'cargo-makepad' not found in PATH" >&2
  echo "       install with: cargo install --path /Users/x/code/makepad/tools/cargo_makepad" >&2
  exit 1
fi

if [[ ! -d "$SDK_PATH" ]]; then
  echo "error: Android SDK not found at: $SDK_PATH" >&2
  echo "       Run ./scripts/build-android-toolchain.sh first," >&2
  echo "       or set MAKEPAD_ANDROID_SDK to the correct path." >&2
  exit 1
fi

echo "==> Android: building release APK (bin=$BIN_NAME, abi=$ABI, pkg=$PKG)"
mkdir -p "$OUT_DIR"

(unset CARGO_TARGET_DIR; cargo makepad android \
  --abi="$ABI" \
  --package-name="$PKG" \
  --app-label="$LABEL" \
  --sdk-path="$SDK_PATH" \
  build -p "$BIN_NAME" --release)

# cargo-makepad places the apk under target/makepad-android-apk/.../<bin>.apk.
BUILT_APK="$(find "$ROOT_DIR/target" -type f -name "*.apk" -newer "$ROOT_DIR/Cargo.toml" 2>/dev/null \
  | grep -v -E '/intermediates?/' \
  | head -1 || true)"

if [[ -z "$BUILT_APK" || ! -f "$BUILT_APK" ]]; then
  echo "error: APK build succeeded but no .apk found under target/" >&2
  exit 1
fi

cp "$BUILT_APK" "$FINAL_APK"
echo "    Android artifact: $FINAL_APK"
