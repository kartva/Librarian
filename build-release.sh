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
tar --transform 's,^,librarian_v1.2/,' -czf $RELEASE_PATH/librarian.tar.gz -C server scripts -C ../target/x86_64-unknown-linux-musl/release librarian 
tar --transform 's,^,librarian_v1.2/,' -czf $RELEASE_PATH/server.tar.gz -C server scripts -C ../target/x86_64-unknown-linux-musl/release server

cd frontend/example_inputs/
unzip -o example_inputs.zip
cd -

cd $RELEASE_PATH
tar -xf librarian.tar 
./librarian_v1.2/librarian ../frontend/example_inputs/example_inputs/ATAC.example.fastq --local --raw --output-dir=.

# check whether the files are non-empty
test -s librarian_heatmap.txt

# check whether the files are recent
test `find librarian_heatmap.txt -mmin -1`

rm librarian_heatmap.txt

cd -