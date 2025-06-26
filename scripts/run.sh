#!/usr/bin/env bash

set -euo pipefail

TARGET=x86_64-kernel

#rustup run nightly cargo build --package walros-kernel $TARGET
#rustup run nightly cargo xrun $TARGET --

$(dirname $0)/build.sh

qemu-system-x86_64 -drive format=raw,file=target/$TARGET/debug/bootimage-walros-kernel.bin -serial stdio