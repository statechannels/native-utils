#!/bin/bash

set -e

if which cargo >/dev/null; then
  echo "Rust (cargo) is installed, building native utilities"
  neon build --release
  cp lib/index.native.js lib/index.js
else
  echo "Rust (cargo) is not installed, falling back to WASM utilities"
  cp lib/index.wasm.js lib/index.js
fi