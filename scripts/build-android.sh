#!/usr/bin/env bash
#
# Build a release APK or AAB for gigu via cargo-makepad.
#
# Env overrides (all optional):
#   BIN_NAME              cargo bin/package to build (default: gigu)
#   ANDROID_PACKAGE_NAME  Android package name      (default: dev.gigu.gigu)
#   ANDROID_APP_LABEL     Android app label         (default: gigu)
#   ANDROID_ABI           comma-list of cargo-makepad ABIs   (default: aarch64,armv7)
#                         valid values: aarch64, x86_64, armv7, i686, all
#   MAKEPAD_ANDROID_SDK   Path to android_33_sdk             (default: ./android_33_sdk)
#
# Output:
#   FORMAT=apk -> dist/android/<bin>-<abi>.apk      (single ABI)
#                 dist/android/<bin>-universal.apk  (multi-ABI)
#   FORMAT=aab -> dist/android/<bin>.aab            (Play Store upload)
#
# Toolchain prerequisite: each ABI needs its rustup target installed. The
# toolchain script defaults to aarch64 only; for additional ABIs run e.g.
#   cargo makepad android --abi=aarch64,armv7 install-toolchain

set -euo pipefail

MAKEPAD_ANDROID_SDK="/Users/x/code/makepad/tools/cargo_makepad/android_33_macos_aarch64"
FORMAT="apk" # apk, aab

# --- AAB signing (only used when FORMAT=aab; leave empty to fall back to the
#     bundled debug.keystore — Play Store will reject an AAB signed with it) ---
ANDROID_KEYSTORE="${ANDROID_KEYSTORE:-}"            # absolute path to .jks/.keystore (Play Store upload key)
ANDROID_KEYSTORE_PASS="${ANDROID_KEYSTORE_PASS:-}"  # store password
ANDROID_KEY_ALIAS="${ANDROID_KEY_ALIAS:-}"          # optional; auto-discovered from <keystore>.makepad sidecar
ANDROID_KEY_PASS="${ANDROID_KEY_PASS:-}"            # defaults to ANDROID_KEYSTORE_PASS if empty

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN_NAME="${BIN_NAME:-gigu}"
PKG="${ANDROID_PACKAGE_NAME:-dev.gigu.gigu}"
LABEL="${ANDROID_APP_LABEL:-gigu}"
ABI="${ANDROID_ABI:-aarch64,armv7}"
SDK_PATH="${MAKEPAD_ANDROID_SDK:-$ROOT_DIR/android_33_sdk}"
OUT_DIR="$ROOT_DIR/dist/android"

if [[ "$FORMAT" != "apk" && "$FORMAT" != "aab" ]]; then
  echo "error: FORMAT must be 'apk' or 'aab' (got '$FORMAT')" >&2
  exit 1
fi

# When ABI is multi-valued, the resulting APK is universal (contains .so for
# every ABI). Use a different filename so single- vs multi-ABI builds don't
# silently overwrite each other. AABs always carry every ABI as separate
# config splits, so the filename doesn't depend on $ABI.
if [[ "$FORMAT" == "aab" ]]; then
  FINAL_ARTIFACT="$OUT_DIR/${BIN_NAME}.aab"
elif [[ "$ABI" == *,* || "$ABI" == "all" ]]; then
  FINAL_ARTIFACT="$OUT_DIR/${BIN_NAME}-universal.apk"
else
  FINAL_ARTIFACT="$OUT_DIR/${BIN_NAME}-${ABI}.apk"
fi

# Each ABI needs its rustup target installed; cargo-makepad surfaces a confusing
# error otherwise. Pre-flight check so the user gets a clear message. Using a
# case here (not an associative array) because macOS ships bash 3.2, which
# silently treats `declare -A` as an indexed array and breaks under `set -u`.
INSTALLED_TARGETS="$(rustup target list --installed 2>/dev/null || true)"
abi_to_rust_target() {
  case "$1" in
    aarch64) echo aarch64-linux-android ;;
    armv7)   echo armv7-linux-androideabi ;;
    x86_64)  echo x86_64-linux-android ;;
    i686)    echo i686-linux-android ;;
    *)       echo "" ;;
  esac
}
expand_abi_list() {
  if [[ "$1" == "all" ]]; then echo "aarch64 x86_64 armv7 i686"; else echo "${1//,/ }"; fi
}
for abi in $(expand_abi_list "$ABI"); do
  rust_target="$(abi_to_rust_target "$abi")"
  if [[ -z "$rust_target" ]]; then
    echo "error: unknown ABI '$abi' (valid: aarch64, x86_64, armv7, i686, all)" >&2
    exit 1
  fi
  if ! grep -qx "$rust_target" <<<"$INSTALLED_TARGETS"; then
    echo "error: rustup target '$rust_target' not installed (needed for ABI '$abi')" >&2
    echo "       run: cargo makepad android --abi=$ABI install-toolchain" >&2
    exit 1
  fi
done

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

echo "==> Android: building release $FORMAT (bin=$BIN_NAME, abi=$ABI, pkg=$PKG)"
mkdir -p "$OUT_DIR"

if [[ "$FORMAT" == "apk" ]]; then
  (unset CARGO_TARGET_DIR; cargo makepad android \
    --abi="$ABI" \
    --package-name="$PKG" \
    --app-label="$LABEL" \
    --sdk-path="$SDK_PATH" \
    build -p "$BIN_NAME" --release)

  # cargo-makepad places the apk under target/makepad-android-apk/.../<bin>.apk.
  BUILT_ARTIFACT="$(find "$ROOT_DIR/target" -type f -name "*.apk" -newer "$ROOT_DIR/Cargo.toml" 2>/dev/null \
    | grep -v -E '/intermediates?/' \
    | head -1 || true)"

  if [[ -z "$BUILT_ARTIFACT" || ! -f "$BUILT_ARTIFACT" ]]; then
    echo "error: APK build succeeded but no .apk found under target/" >&2
    exit 1
  fi
else
  # AAB. Pass keystore options through to cargo-makepad if all required vars
  # are set; otherwise cargo-makepad falls back to its bundled debug.keystore
  # (fine for local install but Play Store will reject — warn loudly).
  KEYSTORE_ARGS=()
  if [[ -n "$ANDROID_KEYSTORE" && -n "$ANDROID_KEYSTORE_PASS" ]]; then
    if [[ ! -f "$ANDROID_KEYSTORE" ]]; then
      echo "error: keystore file not found at: $ANDROID_KEYSTORE" >&2
      exit 1
    fi
    KEYSTORE_ARGS+=(--keystore="$ANDROID_KEYSTORE" --keystore-pass="$ANDROID_KEYSTORE_PASS")
    [[ -n "$ANDROID_KEY_ALIAS" ]] && KEYSTORE_ARGS+=(--keystore-key-alias="$ANDROID_KEY_ALIAS")
    [[ -n "$ANDROID_KEY_PASS"  ]] && KEYSTORE_ARGS+=(--keystore-key-pass="$ANDROID_KEY_PASS")
    echo "    AAB signing: $ANDROID_KEYSTORE"
  else
    echo "==> AAB: signing with bundled debug.keystore (Play Store will reject)"
    echo "         set ANDROID_KEYSTORE + ANDROID_KEYSTORE_PASS at top of script for a real upload key"
  fi

  # bash 3.2 (macOS) treats `"${arr[@]}"` on an empty array as unbound under
  # set -u; expand only when populated.
  (unset CARGO_TARGET_DIR; cargo makepad android \
    --abi="$ABI" \
    --package-name="$PKG" \
    --app-label="$LABEL" \
    --sdk-path="$SDK_PATH" \
    ${KEYSTORE_ARGS[@]+"${KEYSTORE_ARGS[@]}"} \
    build-aab -p "$BIN_NAME" --release)

  # cargo-makepad writes the aab under target/android/makepad-android-aab/.../<label>.aab
  # (label is snakecased — see compile.rs:1928).
  BUILT_ARTIFACT="$(find "$ROOT_DIR/target/android/makepad-android-aab" -type f -name "*.aab" 2>/dev/null \
    | head -1 || true)"

  if [[ -z "$BUILT_ARTIFACT" || ! -f "$BUILT_ARTIFACT" ]]; then
    echo "error: AAB build succeeded but no .aab found under target/makepad-android-aab/" >&2
    exit 1
  fi
fi

cp "$BUILT_ARTIFACT" "$FINAL_ARTIFACT"
echo "    Android artifact: $FINAL_ARTIFACT"
