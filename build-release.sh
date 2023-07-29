#!/bin/bash

set -euxo pipefail

which rustup
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl --bin librarian --bin server

RELEASE_PATH="release"

mkdir -p $RELEASE_PATH

# c creates an archive
# z tells it to use gzip
# f tells it the archive to save to
# -C changes directory, affects subsequent directory changes as well
tar -czf $RELEASE_PATH/librarian.tar.gz -C server scripts -C ../target/x86_64-unknown-linux-musl/release librarian 
tar -czf $RELEASE_PATH/server.tar.gz -C server scripts -C ../target/x86_64-unknown-linux-musl/release server