#!/usr/bin/env sh

set -e

theme="${1:-}"

if [ -z "$theme" ]; then
  echo "error: no theme provided" >&2
  echo "usage: ./install.sh <theme>" >&2
  exit 1
fi

if [ ! -r "$theme" ]; then
  echo "error: theme $theme not found" >&2
  echo "usage: ./install.sh <theme>" >&2
  exit 1
fi

SHIKO_THEME="$theme" cargo build --release

sudo cp ./target/release/shiko /usr/local/bin/shiko

echo "Installed to /usr/local/bin/shiko"
