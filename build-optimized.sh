#!/bin/bash
PROFILE=${1:-release}
TARGET_TRIPLE=${2:-$(rustc -vV | sed -n 's|host: ||p')}
RUSTFLAGS="-C target-cpu=native -C panic=abort" cargo build --profile $PROFILE -Z build-std=std,core,alloc,panic_abort --target $TARGET_TRIPLE
