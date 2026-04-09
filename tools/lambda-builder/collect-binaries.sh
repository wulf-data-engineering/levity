#!/bin/bash
set -e

# Arguments
TARGET_ARCH=${1}
PROFILE=${2}
DEST_DIR=${3}

# Construct the search path
# Default to /var/cargo/target if CARGO_TARGET_DIR is not set
BASE_TARGET_DIR="${CARGO_TARGET_DIR:-/var/cargo/target}"
SEARCH_DIR="$BASE_TARGET_DIR/$TARGET_ARCH/$PROFILE"

echo "--- Collecting binaries ---"
echo "Source: $SEARCH_DIR"
echo "Destination: $DEST_DIR"

if [ ! -d "$SEARCH_DIR" ]; then
  echo "❌ Error: Search directory does not exist. Did the build fail?"
  exit 1
fi

mkdir -p "$DEST_DIR"

# Loop through all files in the top level of the search directory
for file in "$SEARCH_DIR"/*; do
    # 1. Must be a regular file
    # 2. 'file' command must contain 'ELF' (Linux binary)
    # 3. 'file' command must contain 'executable' (to skip .so or .rlib if any)
    if [ -f "$file" ] && file "$file" | grep -q 'ELF' && file "$file" | grep -q 'executable'; then
        echo "Found binary: $(basename "$file")"
        cp "$file" "$DEST_DIR/"
    fi
done

echo "✅ Collection complete."