#!/usr/bin/env bash
#
# Launch the Makepad Studio desktop app — a GUI inspector that talks to our
# running app over WebSocket. Once it's up, run-ios.sh injects STUDIO=127.0.0.1:8001
# into the simulator's environment so the app connects back and forwards logs,
# widget tree, screenshots, etc.
#
# Studio lives in a separate clone of the makepad repo (it isn't part of our
# workspace because it pulls in code_editor / terminal / profiler — a lot of
# stuff we don't need to build for the gigu app itself).
#
# Usage:
#   scripts/studio.sh              # build + run studio in foreground
#   MAKEPAD_REPO=/path scripts/studio.sh
#
# Env overrides:
#   MAKEPAD_REPO   (default: $HOME/code/makepad) — path to the makepad clone
#   STUDIO_PORT    (default: 8001)               — port the hub listens on

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MAKEPAD_REPO="${MAKEPAD_REPO:-$HOME/code/makepad}"
STUDIO_PORT="${STUDIO_PORT:-8001}"

if [[ ! -d "$MAKEPAD_REPO" ]]; then
  echo "error: makepad clone not found at $MAKEPAD_REPO"
  echo "hint:  git clone --branch dev https://github.com/makepad/makepad.git $MAKEPAD_REPO"
  exit 1
fi

echo "Building & launching makepad-studio from $MAKEPAD_REPO"
echo "Hub will listen on 127.0.0.1:$STUDIO_PORT"
echo "(set STUDIO=127.0.0.1:$STUDIO_PORT in the iOS sim to connect)"
echo

# Unset CARGO_TARGET_DIR so studio's build artifacts go into the makepad
# clone's own target/ dir, isolated from our project's build.
exec env -u CARGO_TARGET_DIR \
  cargo run --manifest-path "$MAKEPAD_REPO/Cargo.toml" \
  --release -p makepad-studio &


STUDIO_PORT="${STUDIO_PORT:-8001}"
# IMPORTANT: just `STUDIO=host:port` is not enough. makepad's
# resolve_studio_http() returns empty (= websocket disabled) unless the env
# also carries either a `build` or a `crate` identifier. We forward `crate=gigu`
# so studio's hub can route this app's LogItem / WidgetTreeDump / Screenshot
# messages to the correct project tab.
STUDIO_CRATE_NAME="${1:-gigu}"
export MAKEPAD_STUDIO="127.0.0.1:$STUDIO_PORT?crate=$STUDIO_CRATE_NAME"

# Give studio a moment to start its hub on $STUDIO_PORT before the app tries
# to connect. Without this, the app boots first, the websocket open fails,
# and we fall back to the not-connected path (NSLog only, no studio logs).
echo "Waiting for studio hub on 127.0.0.1:$STUDIO_PORT ..."
for i in {1..30}; do
  if nc -z 127.0.0.1 "$STUDIO_PORT" 2>/dev/null; then
    echo "  studio hub is up"
    break
  fi
  sleep 1
done

bash "$ROOT_DIR/scripts/run-ios.sh" "$@"
