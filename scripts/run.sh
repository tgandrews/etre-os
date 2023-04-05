#!/bin/bash
set +eux

cargo bootimage

qemu-system-x86_64 \
  -machine pc \
  -m 1024M \
  -drive format=raw,file=target/x86_64-unknown-etre/debug/etre-os.bin