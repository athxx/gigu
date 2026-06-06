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

# --- Release signing. Keystore + credentials live in signature/android/, which
#     is .gitignored. Credentials are parsed from jks_password.txt (shipped
#     alongside every rotated jks) so rotating the key only requires swapping
#     the two files — no script edit. Env vars still override for CI. ---
SIGNATURE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/signature/android"
ANDROID_KEYSTORE="${ANDROID_KEYSTORE:-$(ls "$SIGNATURE_DIR"/*.jks 2>/dev/null | head -1)}"
ANDROID_PASSWORD_FILE="${ANDROID_PASSWORD_FILE:-$SIGNATURE_DIR/jks_password.txt}"

# Pull credentials out of jks_password.txt. Lines look like
#   "store pass<中文>: N9AHeGSG"
#   "key alias<中文>: ym061734"
#   "key pass<中文>: N9AHeGSG"
# (the file is GBK with Chinese annotations between the English label and the
# colon). We anchor on the leading English label only, then take everything
# after the LAST colon on the line, trimming whitespace + CR.
parse_pwd_field() {
  local key="$1" file="$2"
  # LC_ALL=C: file is GBK; force byte-level so BSD sed doesn't choke on
  # non-UTF-8 bytes ("illegal byte sequence").
  LC_ALL=C grep -i "^${key}" "$file" 2>/dev/null \
    | head -1 \
    | LC_ALL=C sed -E 's/.*:[[:space:]]*//; s/[[:space:]\r]+$//'
}
if [[ -z "${ANDROID_KEYSTORE_PASS:-}" || -z "${ANDROID_KEY_ALIAS:-}" || -z "${ANDROID_KEY_PASS:-}" ]]; then
  if [[ ! -f "$ANDROID_PASSWORD_FILE" ]]; then
    echo "error: credential file not found: $ANDROID_PASSWORD_FILE" >&2
    echo "       expected lines: 'store pass: ...', 'key alias: ...', 'key pass: ...'" >&2
    echo "       (or set ANDROID_KEYSTORE_PASS / ANDROID_KEY_ALIAS / ANDROID_KEY_PASS)" >&2
    exit 1
  fi
  ANDROID_KEYSTORE_PASS="${ANDROID_KEYSTORE_PASS:-$(parse_pwd_field "store pass" "$ANDROID_PASSWORD_FILE")}"
  ANDROID_KEY_ALIAS="${ANDROID_KEY_ALIAS:-$(parse_pwd_field "key alias"  "$ANDROID_PASSWORD_FILE")}"
  ANDROID_KEY_PASS="${ANDROID_KEY_PASS:-$(parse_pwd_field "key pass"   "$ANDROID_PASSWORD_FILE")}"
fi
if [[ -z "$ANDROID_KEYSTORE_PASS" || -z "$ANDROID_KEY_ALIAS" || -z "$ANDROID_KEY_PASS" ]]; then
  echo "error: failed to resolve signing credentials from $ANDROID_PASSWORD_FILE" >&2
  echo "       expected lines: 'store pass: ...', 'key alias: ...', 'key pass: ...'" >&2
  exit 1
fi

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

# Verify the keystore exists before building anything — both APK and AAB paths
# need it now, and a build that succeeds but can't be signed wastes minutes.
if [[ ! -f "$ANDROID_KEYSTORE" ]]; then
  echo "error: keystore not found at: $ANDROID_KEYSTORE" >&2
  echo "       set ANDROID_KEYSTORE to a valid .jks/.keystore path" >&2
  exit 1
fi

echo "==> Android: building release $FORMAT (bin=$BIN_NAME, abi=$ABI, pkg=$PKG)"
echo "    signing: $ANDROID_KEYSTORE (alias=$ANDROID_KEY_ALIAS)"
mkdir -p "$OUT_DIR"

# Shared keystore args for cargo-makepad's AAB path.
KEYSTORE_ARGS=(
  --keystore="$ANDROID_KEYSTORE"
  --keystore-pass="$ANDROID_KEYSTORE_PASS"
  --keystore-key-alias="$ANDROID_KEY_ALIAS"
  --keystore-key-pass="$ANDROID_KEY_PASS"
)

if [[ "$FORMAT" == "apk" ]]; then
  (unset CARGO_TARGET_DIR; cargo makepad android \
    --abi="$ABI" \
    --package-name="$PKG" \
    --app-label="$LABEL" \
    --sdk-path="$SDK_PATH" \
    build -p "$BIN_NAME" --release)

  # cargo-makepad places the apk under target/android/makepad-android-apk/.../<bin>.apk.
  BUILT_ARTIFACT="$(find "$ROOT_DIR/target" -type f -name "*.apk" -newer "$ROOT_DIR/Cargo.toml" 2>/dev/null \
    | grep -v -E '/intermediates?/' \
    | head -1 || true)"

  if [[ -z "$BUILT_ARTIFACT" || ! -f "$BUILT_ARTIFACT" ]]; then
    echo "error: APK build succeeded but no .apk found under target/" >&2
    exit 1
  fi

  # cargo-makepad's `build` subcommand emits an UNSIGNED apk (only `build-aab`
  # signs). Sign in-place with apksigner from the SDK build-tools so the apk
  # is installable.
  APKSIGNER_JAR="$SDK_PATH/build-tools/33.0.1/lib/apksigner.jar"
  if [[ ! -f "$APKSIGNER_JAR" ]]; then
    echo "error: apksigner.jar not found at $APKSIGNER_JAR" >&2
    exit 1
  fi
  echo "==> APK: signing with $ANDROID_KEYSTORE"
  java -jar "$APKSIGNER_JAR" sign \
    --ks "$ANDROID_KEYSTORE" \
    --ks-pass "pass:$ANDROID_KEYSTORE_PASS" \
    --ks-key-alias "$ANDROID_KEY_ALIAS" \
    --key-pass "pass:$ANDROID_KEY_PASS" \
    "$BUILT_ARTIFACT"
else
  # bash 3.2 (macOS) treats `"${arr[@]}"` on an empty array as unbound under
  # set -u; KEYSTORE_ARGS is always populated here so direct expansion is fine.
  (unset CARGO_TARGET_DIR; cargo makepad android \
    --abi="$ABI" \
    --package-name="$PKG" \
    --app-label="$LABEL" \
    --sdk-path="$SDK_PATH" \
    "${KEYSTORE_ARGS[@]}" \
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
