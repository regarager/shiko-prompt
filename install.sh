#!/usr/bin/env sh

set -e

SHIKO_THEME="$1" cargo build --release
sudo cp ./target/release/shiko /usr/local/bin

echo "Installed to /usr/local/bin/shiko"
