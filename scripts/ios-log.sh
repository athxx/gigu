#!/usr/bin/env bash
#
# Tail logs from the booted iOS simulator filtered by process / app name.
#
# Usage:
#   scripts/ios-log.sh                 # process == "gigu"
#   scripts/ios-log.sh map_demo        # process == "map_demo"
#
# Env overrides:
#   MAKEPAD_IOS_APP   (default: gigu)

set -euo pipefail

PROCESS_NAME="${1:-gigu}"
APP_NAME="${MAKEPAD_IOS_APP:-gigu}"

if ! command -v xcrun >/dev/null 2>&1; then
  echo "error: xcrun not found; install Xcode command line tools"
  exit 1
fi

BOOTED_DEVICE="$(xcrun simctl list devices booted 2>/dev/null | grep -E 'Booted' || true)"
if [[ -z "$BOOTED_DEVICE" ]]; then
  echo "error: no booted iOS simulator found"
  echo "hint: open Simulator.app and boot a device first"
  exit 1
fi

echo "Streaming logs for process '${PROCESS_NAME}' or app '${APP_NAME}'..."
echo "Booted device: ${BOOTED_DEVICE}"
echo

exec xcrun simctl spawn booted log stream \
  --style compact \
  --predicate "process == \"${PROCESS_NAME}\" OR process == \"${APP_NAME}\""
