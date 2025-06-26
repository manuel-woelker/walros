#!/usr/bin/env bash

set -euo pipefail

TARGET=--target=x86_64-kernel.json
export CARGO_MANIFEST_DIR=$(dirname $0)/..
#rustup run nightly cargo build --package walros-kernel $TARGET
rustup run nightly cargo bootimage $TARGET