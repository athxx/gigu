#!/usr/bin/env bash
#
# Build & run the gigu app on the booted iOS simulator via cargo-makepad.
#
# The installed `cargo-makepad` generates a minimal Info.plist that
# is missing the usage-description keys required by iOS when the app touches
# camera / location / microphone APIs. That makes the freshly launched app
# crash immediately with EXC_CRASH (SIGABRT) and a TCC termination reason:
#
#   "This app has crashed because it attempted to access privacy-sensitive
#    data without a usage description."
#
# To work around that without forcing a re-install of cargo-makepad from the
# `dev` branch, this script:
#
#   1. Lets `cargo makepad apple ios run-sim` build & install the .app bundle on
#      the booted simulator (the launch that follows will crash; that's OK).
#   2. Patches the installed bundle's Info.plist with the missing usage
#      description keys.
#   3. Re-launches the app via `xcrun simctl launch`.
#
# Usage:
#   scripts/run-ios.sh                # runs the default bin `gigu`
#   scripts/run-ios.sh map_demo       # runs a different bin / package
#
# Env overrides:
#   MAKEPAD_IOS_ORG    (default: dev.gigu)
#   MAKEPAD_IOS_APP    (default: gigu)
#   MAKEPAD_BUNDLE_ID  (default: ${MAKEPAD_IOS_ORG}.${MAKEPAD_IOS_APP})

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="${1:-gigu}"
ORG_NAME="${MAKEPAD_IOS_ORG:-dev.gigu}"
APP_NAME="${MAKEPAD_IOS_APP:-gigu}"
BUNDLE_ID="${MAKEPAD_BUNDLE_ID:-${ORG_NAME}.${APP_NAME}}"

if ! command -v cargo-makepad >/dev/null 2>&1; then
  echo "error: cargo-makepad not found in PATH"
  echo "hint:  cargo install cargo-makepad"
  exit 1
fi

if ! command -v xcrun >/dev/null 2>&1; then
  echo "error: xcrun not found; install Xcode command line tools"
  exit 1
fi

BOOTED="$(xcrun simctl list devices booted 2>/dev/null | grep -E 'Booted' || true)"
if [[ -z "$BOOTED" ]]; then
  echo "error: no booted iOS simulator found"
  echo "hint:  open Simulator.app and boot a device (e.g. iPhone 16e) first"
  exit 1
fi

echo "Booted simulator: $BOOTED"
echo "Bundle id:        $BUNDLE_ID"
echo "Running:          cargo makepad apple ios --org=$ORG_NAME --app=$APP_NAME run-sim -p $BIN_NAME"
echo

# Step 0: terminate + uninstall any previous copy so the simulator's
# LaunchServices doesn't reuse a cached bundle / stale Info.plist / stale
# resources. Without this, a freshly-built app sometimes shows the previous
# UI because launchd picks up the cached install.
echo "Uninstalling any previous copy of $BUNDLE_ID ..."
xcrun simctl terminate booted "$BUNDLE_ID" >/dev/null 2>&1 || true
xcrun simctl uninstall booted "$BUNDLE_ID" >/dev/null 2>&1 || true

# Step 1: build, install, and (best-effort) launch. We deliberately ignore the
# exit status because cargo-makepad does not propagate a failed launch.
#
# IMPORTANT: unset CARGO_TARGET_DIR so cargo writes the build artifacts into
# the project's `target/` dir, which is where `cargo makepad ios run-sim`
# looks for the binary it bundles into the .app. If the env exports
# CARGO_TARGET_DIR (e.g. /tmp/rust_tmp), cargo builds there, cargo-makepad
# reads from `target/`, and the simulator ends up running a stale binary
# or fails outright. Unset only for this subshell — leave the user's env
# untouched.
set +e
(unset CARGO_TARGET_DIR; cargo makepad apple ios --org="$ORG_NAME" --app="$APP_NAME" run-sim -p "$BIN_NAME")
set -e

# Step 2: terminate any running instance so we can relaunch cleanly.
xcrun simctl terminate booted "$BUNDLE_ID" >/dev/null 2>&1 || true

# Step 3: locate the installed bundle and patch Info.plist with the missing
# usage description keys. `simctl get_app_container booted <bundle_id>` prints
# the absolute path to the installed .app on the booted device.
#
# `cargo makepad apple ios run-sim` installs *and* launches the app itself, then
# exits. LaunchServices registers the freshly installed bundle asynchronously,
# so get_app_container can briefly return empty right after run-sim returns.
# Retry a few times before giving up.
APP_PATH=""
for _ in 1 2 3 4 5 6 7 8 9 10; do
  APP_PATH="$(xcrun simctl get_app_container booted "$BUNDLE_ID" 2>/dev/null || true)"
  [[ -n "$APP_PATH" && -d "$APP_PATH" ]] && break
  sleep 0.5
done
if [[ -z "$APP_PATH" || ! -d "$APP_PATH" ]]; then
  echo "error: could not locate installed bundle for $BUNDLE_ID on the simulator"
  echo "       run-sim above probably failed before installing the app."
  exit 1
fi

PLIST="$APP_PATH/Info.plist"
echo
echo "Patching Info.plist with iOS privacy usage descriptions:"
echo "  $PLIST"

# `plutil -replace` creates the key if missing, or overwrites if present, so
# this is idempotent across re-runs.
plutil -replace NSCameraUsageDescription \
  -string "gigu uses the camera for the camera demo page." "$PLIST"
plutil -replace NSMicrophoneUsageDescription \
  -string "gigu uses the microphone alongside the camera demo." "$PLIST"
plutil -replace NSLocationWhenInUseUsageDescription \
  -string "gigu uses your location for the location demo page." "$PLIST"
plutil -replace NSLocationAlwaysAndWhenInUseUsageDescription \
  -string "gigu uses your location for the location demo page." "$PLIST"
plutil -replace NSLocationUsageDescription \
  -string "gigu uses your location for the location demo page." "$PLIST"
# AVFoundation logs a warning ("Add NSCameraUseContinuityCameraDeviceType to
# your Info.plist...") whenever AVCaptureDeviceTypeContinuityCamera is queried.
# Setting this to false opts out and silences the warning on the simulator.
plutil -replace NSCameraUseContinuityCameraDeviceType -bool NO "$PLIST"

# Step 4: re-install the patched bundle so the simulator's launchservicesd
# refreshes its cached Info.plist. WITHOUT this step TCC keeps using the
# pre-patch metadata captured at the original install (which has no usage
# description keys), and the next launch crashes with:
#   namespace=TCC ... NSCameraUsageDescription key ...
# even though the on-disk plist now contains the key.
echo
echo "Re-installing patched bundle so launchservicesd picks up new Info.plist ..."
xcrun simctl install booted "$APP_PATH"

# Step 5: relaunch the patched app on the simulator.
#
# `simctl launch` propagates env vars to the launched app via the
# `SIMCTL_CHILD_<NAME>` prefix (the prefix is stripped, the rest becomes a
# normal env var inside the simulator process).
#
# If MAKEPAD_STUDIO is set (e.g. 127.0.0.1:8001), tell the app to connect to
# the studio hub. The host's localhost is reachable from the simulator, so a
# studio running on the Mac at that port will receive the websocket. When
# unset, the app prints "studio websocket disabled: empty studio_http" and
# runs normally — that's the default.
echo
echo "Relaunching $BUNDLE_ID ..."
if [[ -n "${MAKEPAD_STUDIO:-}" ]]; then
  echo "  forwarding STUDIO=$MAKEPAD_STUDIO to the simulator process"
  SIMCTL_CHILD_STUDIO="$MAKEPAD_STUDIO" \
    xcrun simctl launch booted "$BUNDLE_ID"
else
  xcrun simctl launch booted "$BUNDLE_ID"
fi

echo
echo "Done. Tail logs with: scripts/ios-log.sh"
echo "Studio inspector:    scripts/studio.sh   (then re-run with MAKEPAD_STUDIO=127.0.0.1:8001 $0)"
