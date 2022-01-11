#!/bin/bash

set -euxo pipefail

npm --version && \
	cargo --version && \
	wasm-pack --version

cd frontend
npm install
wasm-pack build
npm run build
cd ..

cd server
LIBRARIAN_INDEX_PATH="../frontend/dist/"
LIBRARIAN_PORT="8186"
cargo run --release
cd ..
