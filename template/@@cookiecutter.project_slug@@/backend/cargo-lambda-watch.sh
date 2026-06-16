#!/bin/bash
cd "$(dirname "$0")"

FORCE=0
for arg in "$@"; do
  if [[ "$arg" == "-f" || "$arg" == "--force" ]]; then
    FORCE=1
  fi
done

# Load the `.env` from the project root if it exists
if [ -f ../.env ]; then
  set -a
  source ../.env
  set +a
fi

PORT=${BACKEND_PORT:-9000}
PIDS=$(lsof -t -i:$PORT)

if [ -n "$PIDS" ]; then
  if [ "$FORCE" -eq 1 ]; then
    echo "Port $PORT is blocked by PID(s): $PIDS. Gracefully stopping them..."
    kill -2 $PIDS 2>/dev/null || true
  else
    echo "Error: Port $PORT is already in use by PID(s): $PIDS."
    echo "Run this script with --force (or -f) to automatically stop the blocking process."
    exit 1
  fi
fi

BIND_ADDRESS="0.0.0.0"
if [ "$CI" = "true" ]; then
  BIND_ADDRESS="127.0.0.1"
fi

cargo build || exit 1 # Always run this first to ensure that the code compiles and starts fast
cargo lambda watch -A $BIND_ADDRESS -P $PORT
