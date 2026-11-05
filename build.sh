#!/usr/bin/env bash
set -e

which qemu-system-x86_64

# use bootimage to link kernel with bootloader
cargo bootimage

qemu-system-x86_64 -drive format=raw,file=target/x86_64-dos/debug/bootimage-dos.bin
