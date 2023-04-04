#!/bin/bash
set +eux

qemu-system-aarch64 -machine virt \
  -m 1024M \
  -cpu cortex-a53 \
  -nographic \
  -kernel target/aarch64-unknown-none/debug/etre-os