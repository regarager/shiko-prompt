#!/usr/bin/env sh

set -e

theme="${1:-themes/default.json}"
SHIKO_THEME="$theme" cargo build --release
sudo cp ./target/release/shiko /usr/local/bin

echo "Installed to /usr/local/bin/shiko"
