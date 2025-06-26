#!/usr/bin/env bash

set -euo pipefail

TARGET=--target=x86_64-kernel.json

#rustup run nightly cargo build --package walros-kernel $TARGET
rustup run nightly cargo bootimage $TARGET