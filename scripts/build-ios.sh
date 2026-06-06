#!/usr/bin/env bash
#
# Build a signed App Store .ipa for gigu.
#
# Workflow:
#   1. let cargo-makepad produce a release simulator .app — used only as a
#      template (Info.plist, AppIcon, resources).
#   2. cargo build --target aarch64-apple-ios --release for the device binary.
#   3. swap the template's sim binary for the device binary, embed the
#      provisioning profile, codesign with the distribution identity, and
#      package as Payload/<App>.app -> dist/ios/<bin>.ipa.
#
# Output: dist/ios/<bin>.ipa  (upload via Transporter or `xcrun altool`).
#
# Env overrides (all optional except signing):
#   BIN_NAME                  cargo bin/package          (default: gigu)
#   MAKEPAD_IOS_ORG           iOS org name               (default: dev.gigu)
#   MAKEPAD_IOS_APP           iOS product name           (default: gigu)
#
# --- Signing (REQUIRED for a usable .ipa, leave empty to skip codesign) ---
IOS_CODESIGN_IDENTITY="${IOS_CODESIGN_IDENTITY:-}"        # e.g. "Apple Distribution: Your Name (ABCDE12345)"
IOS_PROVISIONING_PROFILE="${IOS_PROVISIONING_PROFILE:-}"  # absolute path to a .mobileprovision (App Store distribution)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="${BIN_NAME:-gigu}"
ORG="${MAKEPAD_IOS_ORG:-dev.gigu}"
APP="${MAKEPAD_IOS_APP:-gigu}"
OUT_DIR="$ROOT_DIR/dist/ios"
STAGE_DIR="$ROOT_DIR/target/ipa-stage"

DEVICE_TARGET="aarch64-apple-ios"
SIM_APP="$ROOT_DIR/target/makepad-ios-app/aarch64-apple-ios-sim/release/${APP}.app"
DEVICE_BIN="$ROOT_DIR/target/${DEVICE_TARGET}/release/${BIN_NAME}"
IPA_PATH="$OUT_DIR/${BIN_NAME}.ipa"

if ! command -v cargo-makepad >/dev/null 2>&1; then
  echo "error: 'cargo-makepad' not found in PATH" >&2
  echo "       install with: cargo install --path /Users/x/code/makepad/tools/cargo_makepad" >&2
  exit 1
fi

if ! command -v xcrun >/dev/null 2>&1; then
  echo "error: 'xcrun' not found — install Xcode command line tools" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

# --- 1. produce .app template via cargo-makepad's release sim build ---------
echo "==> iOS: building .app template (release sim, bin=$BIN_NAME, org=$ORG, app=$APP)"
(unset CARGO_TARGET_DIR; cargo makepad apple ios \
  --org="$ORG" --app="$APP" \
  build -p "$BIN_NAME" --release)

if [[ ! -d "$SIM_APP" ]]; then
  echo "error: expected template .app not found at $SIM_APP" >&2
  echo "       cargo-makepad may have produced a different layout — check target/makepad-ios-app/" >&2
  exit 1
fi

# --- 2. build the device binary ---------------------------------------------
echo "==> iOS: building device binary ($DEVICE_TARGET, release)"
(unset CARGO_TARGET_DIR; \
  IPHONEOS_DEPLOYMENT_TARGET=15.0 AWS_LC_SYS_CMAKE_BUILDER=1 \
  cargo build --target="$DEVICE_TARGET" --release -p "$BIN_NAME")

if [[ ! -f "$DEVICE_BIN" ]]; then
  echo "error: device binary not found at $DEVICE_BIN" >&2
  exit 1
fi

# --- 3. assemble device .app from template + device binary ------------------
DEVICE_APP="$STAGE_DIR/${APP}.app"
echo "==> iOS: assembling device .app at $DEVICE_APP"
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR"
cp -R "$SIM_APP" "$DEVICE_APP"
# replace sim binary with device binary; cargo-makepad names the binary
# whatever the cargo bin name is, which equals $BIN_NAME for gigu.
cp -f "$DEVICE_BIN" "$DEVICE_APP/$BIN_NAME"
chmod +x "$DEVICE_APP/$BIN_NAME"
# strip any sim-side codesigning leftovers
rm -rf "$DEVICE_APP/_CodeSignature"

# --- 4. codesign (if signing env is set) ------------------------------------
if [[ -z "$IOS_CODESIGN_IDENTITY" || -z "$IOS_PROVISIONING_PROFILE" ]]; then
  echo "==> iOS: signing skipped (IOS_CODESIGN_IDENTITY and/or IOS_PROVISIONING_PROFILE empty)"
  echo "         the .ipa will NOT be installable — fill in the env vars at the top of this script."
else
  if [[ ! -f "$IOS_PROVISIONING_PROFILE" ]]; then
    echo "error: provisioning profile not found at: $IOS_PROVISIONING_PROFILE" >&2
    exit 1
  fi

  echo "==> iOS: embedding provisioning profile"
  cp "$IOS_PROVISIONING_PROFILE" "$DEVICE_APP/embedded.mobileprovision"

  # extract team id + entitlements from the profile (mobileprovision is a CMS
  # blob wrapping a plist).
  PROFILE_PLIST="$STAGE_DIR/profile.plist"
  ENTITLEMENTS_PLIST="$STAGE_DIR/entitlements.plist"
  security cms -D -i "$IOS_PROVISIONING_PROFILE" -o "$PROFILE_PLIST"
  /usr/libexec/PlistBuddy -x -c 'Print :Entitlements' "$PROFILE_PLIST" > "$ENTITLEMENTS_PLIST"

  echo "==> iOS: codesigning $BIN_NAME and ${APP}.app with: $IOS_CODESIGN_IDENTITY"
  # sign nested binary first, then the .app bundle.
  codesign --force --timestamp=none --sign "$IOS_CODESIGN_IDENTITY" \
    "$DEVICE_APP/$BIN_NAME"
  codesign --force --timestamp=none --sign "$IOS_CODESIGN_IDENTITY" \
    --entitlements "$ENTITLEMENTS_PLIST" --generate-entitlement-der \
    "$DEVICE_APP"
fi

# --- 5. package as Payload/<App>.app inside ${BIN_NAME}.ipa ------------------
echo "==> iOS: packaging $IPA_PATH"
PAYLOAD_DIR="$STAGE_DIR/Payload"
rm -rf "$PAYLOAD_DIR"
mkdir -p "$PAYLOAD_DIR"
cp -R "$DEVICE_APP" "$PAYLOAD_DIR/"

rm -f "$IPA_PATH"
( cd "$STAGE_DIR" && zip -qry "$IPA_PATH" Payload )

echo "    iOS artifact: $IPA_PATH"
if [[ -z "$IOS_CODESIGN_IDENTITY" || -z "$IOS_PROVISIONING_PROFILE" ]]; then
  echo "    (unsigned — fill IOS_CODESIGN_IDENTITY + IOS_PROVISIONING_PROFILE at top of script)"
else
  echo "    upload via Transporter, or: xcrun altool --upload-app -f \"$IPA_PATH\" -t ios ..."
fi
