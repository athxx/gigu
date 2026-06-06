#!/usr/bin/env bash
#
# Build a debug APK, install it onto a connected device/emulator, launch it,
# and follow logcat. Mirrors run.sh but for Android.
#
# Flags:
#   --reuse   skip the build, install whatever's already in dist/android/
#             (gigu-universal.apk preferred, else gigu-<abi>.apk)
#   --no-log  install + launch but don't tail logcat
#
# Env overrides (all optional):
#   BIN_NAME              cargo bin/package          (default: gigu)
#   ANDROID_PACKAGE_NAME  Android package name       (default: dev.gigu.gigu)
#   ANDROID_APP_LABEL     Android app label          (default: gigu)
#   ANDROID_ABI           cargo-makepad ABI list     (default: aarch64)
#   MAKEPAD_ANDROID_SDK   path to android_33_sdk     (default below)

set -euo pipefail

MAKEPAD_ANDROID_SDK="${MAKEPAD_ANDROID_SDK:-/Users/x/code/makepad/tools/cargo_makepad/android_33_macos_aarch64}"
ADB="$MAKEPAD_ANDROID_SDK/platform-tools/adb"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="${BIN_NAME:-gigu}"
PKG="${ANDROID_PACKAGE_NAME:-dev.gigu.gigu}"
LABEL="${ANDROID_APP_LABEL:-gigu}"
ABI="${ANDROID_ABI:-aarch64}"
OUT_DIR="$ROOT_DIR/dist/android"
LAUNCHER="$PKG/$PKG.MakepadApp"

REUSE=0
TAIL_LOG=1
for arg in "$@"; do
  case "$arg" in
    --reuse)  REUSE=1 ;;
    --no-log) TAIL_LOG=0 ;;
    *) echo "unknown flag: $arg (valid: --reuse, --no-log)" >&2; exit 1 ;;
  esac
done

if [[ ! -x "$ADB" ]]; then
  echo "error: adb not found at $ADB" >&2
  echo "       set MAKEPAD_ANDROID_SDK or run ./scripts/build-android-toolchain.sh" >&2
  exit 1
fi

# --- 1. pick a device ------------------------------------------------------
DEVICES="$("$ADB" devices | awk 'NR>1 && $2=="device" {print $1}')"
DEVICE_COUNT="$(printf '%s\n' "$DEVICES" | grep -c .)"
if [[ "$DEVICE_COUNT" -eq 0 ]]; then
  echo "error: no adb devices online — start an emulator or plug in a phone" >&2
  exit 1
elif [[ "$DEVICE_COUNT" -gt 1 ]]; then
  echo "error: multiple adb devices online; set ANDROID_SERIAL=<serial>" >&2
  echo "$DEVICES" | sed 's/^/       /' >&2
  exit 1
fi
SERIAL="$DEVICES"
DEVICE_ABI="$("$ADB" -s "$SERIAL" shell getprop ro.product.cpu.abi | tr -d '\r')"
echo "==> device: $SERIAL (abi=$DEVICE_ABI)"

# --- 2. build (unless --reuse) ---------------------------------------------
if [[ "$REUSE" -eq 1 ]]; then
  # Prefer universal, then per-ABI matching the device.
  if   [[ -f "$OUT_DIR/${BIN_NAME}-universal.apk" ]]; then APK="$OUT_DIR/${BIN_NAME}-universal.apk"
  elif [[ "$DEVICE_ABI" == "arm64-v8a" && -f "$OUT_DIR/${BIN_NAME}-aarch64.apk" ]]; then APK="$OUT_DIR/${BIN_NAME}-aarch64.apk"
  elif [[ "$DEVICE_ABI" == "armeabi-v7a" && -f "$OUT_DIR/${BIN_NAME}-armv7.apk" ]]; then APK="$OUT_DIR/${BIN_NAME}-armv7.apk"
  elif [[ "$DEVICE_ABI" == "x86_64" && -f "$OUT_DIR/${BIN_NAME}-x86_64.apk" ]]; then APK="$OUT_DIR/${BIN_NAME}-x86_64.apk"
  else
    echo "error: --reuse but no matching APK in $OUT_DIR for abi=$DEVICE_ABI" >&2
    exit 1
  fi
  echo "==> reusing $APK"
else
  echo "==> building debug apk (bin=$BIN_NAME, abi=$ABI, pkg=$PKG)"
  mkdir -p "$OUT_DIR"
  (unset CARGO_TARGET_DIR; cargo makepad android \
    --abi="$ABI" \
    --package-name="$PKG" \
    --app-label="$LABEL" \
    --sdk-path="$MAKEPAD_ANDROID_SDK" \
    build -p "$BIN_NAME")

  # cargo-makepad debug apk lands under target/android/makepad-android-apk/.../<bin>.apk
  APK="$(find "$ROOT_DIR/target" -type f -name "*.apk" -newer "$ROOT_DIR/Cargo.toml" 2>/dev/null \
    | grep -v -E '/intermediates?/' | head -1 || true)"
  if [[ -z "$APK" || ! -f "$APK" ]]; then
    echo "error: build succeeded but no .apk found under target/" >&2
    exit 1
  fi
  cp "$APK" "$OUT_DIR/${BIN_NAME}-debug.apk"
  APK="$OUT_DIR/${BIN_NAME}-debug.apk"
  echo "    apk: $APK"
fi

# --- 3. sign with the bundled debug keystore -------------------------------
# cargo-makepad's `build` subcommand emits an UNSIGNED apk; Android refuses to
# install it (INSTALL_PARSE_FAILED_NO_CERTIFICATES). Sign in-place with the
# debug keystore that ships next to cargo-makepad. (build-aab handles signing
# itself, so this is only needed on the apk path.)
APKSIGNER_JAR="$MAKEPAD_ANDROID_SDK/build-tools/33.0.1/lib/apksigner.jar"
DEBUG_KEYSTORE="$(dirname "$MAKEPAD_ANDROID_SDK")/debug.keystore"
if ! unzip -l "$APK" 2>/dev/null | grep -q "META-INF/.*\.\(RSA\|DSA\|EC\)"; then
  if [[ ! -f "$APKSIGNER_JAR" ]]; then
    echo "error: apksigner.jar not found at $APKSIGNER_JAR" >&2
    exit 1
  fi
  if [[ ! -f "$DEBUG_KEYSTORE" ]]; then
    echo "error: debug keystore not found at $DEBUG_KEYSTORE" >&2
    exit 1
  fi
  echo "==> signing apk with debug keystore"
  java -jar "$APKSIGNER_JAR" sign \
    --ks "$DEBUG_KEYSTORE" \
    --ks-pass pass:android \
    --key-pass pass:android \
    --ks-key-alias androiddebugkey \
    "$APK"
fi

# --- 4. install ------------------------------------------------------------
echo "==> installing to $SERIAL"
# -r reinstall, -d allow downgrade (so a newer-then-older swap doesn't fail),
# -t allow test packages (debug builds).
if ! "$ADB" -s "$SERIAL" install -r -d -t "$APK" 2>&1 | tee /tmp/gigu-install.log | tail -5; then
  echo "error: adb install failed — see /tmp/gigu-install.log" >&2
  # Common cause: signature mismatch with a previously-installed copy.
  if grep -q "INSTALL_FAILED_UPDATE_INCOMPATIBLE\|signatures do not match" /tmp/gigu-install.log; then
    echo "       cause: a copy of $PKG signed with a different key is already installed." >&2
    echo "       fix:   $ADB -s $SERIAL uninstall $PKG && rerun" >&2
  fi
  exit 1
fi

# --- 4. launch -------------------------------------------------------------
echo "==> launching $LAUNCHER"
"$ADB" -s "$SERIAL" shell am start -n "$LAUNCHER" >/dev/null

# --- 5. follow logs --------------------------------------------------------
if [[ "$TAIL_LOG" -eq 1 ]]; then
  PID=""
  for _ in 1 2 3 4 5 6 7 8 9 10; do
    PID="$("$ADB" -s "$SERIAL" shell pidof "$PKG" | tr -d '\r')"
    [[ -n "$PID" ]] && break
    sleep 0.3
  done
  if [[ -z "$PID" ]]; then
    echo "==> couldn't find pid; tailing unfiltered logcat (Ctrl-C to stop)"
    "$ADB" -s "$SERIAL" logcat -T 50
  else
    echo "==> tailing logcat for pid=$PID (Ctrl-C to stop)"
    "$ADB" -s "$SERIAL" logcat --pid="$PID" -T 50
  fi
fi
