#!/bin/bash
# Helper script to run Nodoka with correct VLC library path

export DYLD_LIBRARY_PATH=/Applications/VLC.app/Contents/MacOS/lib:$DYLD_LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/lib:/usr/local/lib:$LD_LIBRARY_PATH

if [ "$1" = "test" ]; then
    cargo test --all -- --nocapture
elif [ "$1" = "build" ]; then
    cargo build --release
elif [ "$1" = "run" ]; then
    cargo run --release
else
    ./target/release/nodoka
fi
