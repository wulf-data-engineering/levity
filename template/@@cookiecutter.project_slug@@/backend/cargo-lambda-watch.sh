#!/bin/bash
cd "$(dirname "$0")"

# Load the `.env` from the project root if it exists
if [ -f ../.env ]; then
  set -a
  source ../.env
  set +a
fi

cargo build # Always run this first to ensure that the code compiles and starts fast
cargo lambda watch -P ${BACKEND_PORT:-9000}
