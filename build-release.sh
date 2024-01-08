#!/bin/bash

set -euxo pipefail

which rustup
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl --bin librarian --bin server

RELEASE_PATH="release"

mkdir -p $RELEASE_PATH

# get the version number
VERSION=$(cargo run --release --bin librarian -- --version | awk '{print $NF}')
DIRNAME="librarian_v$VERSION"

# c creates an archive
# z tells it to use gzip
# f tells it the archive to save to
# s sends the filename to sed with the given commands
# -C changes directory, affects subsequent directory changes as well
tar --overwrite --transform "s,^,$DIRNAME/," -czf $RELEASE_PATH/librarian.tar.gz -C server scripts -C ../target/x86_64-unknown-linux-musl/release librarian 
tar --overwrite --transform "s,^,$DIRNAME/," -czf $RELEASE_PATH/server.tar.gz -C server scripts -C ../target/x86_64-unknown-linux-musl/release server

cd frontend/example_inputs/
unzip -o example_inputs.zip # -o overwrites existing files
cd -

cd $RELEASE_PATH
tar -xf librarian.tar 
./$DIRNAME/librarian ../frontend/example_inputs/example_inputs/ATAC.example.fastq --local --raw --output-dir=.

# check whether the files are non-empty
test -s librarian_heatmap.txt

# check whether the files are recent
test `find librarian_heatmap.txt -mmin -1`

rm librarian_heatmap.txt

cd -