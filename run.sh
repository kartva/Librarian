#!/bin/bash

set -euxo pipefail

npm --version && \
	cargo --version && \
	wasm-pack --version

cd frontend
npm install
npm run build
cd ..

cd server
LIBRARIAN_INDEX_PATH="../frontend/dist/"
LIBRARIAN_PORT="8186"
cargo run --bin server --release
cd ..
