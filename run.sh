#!/bin/bash

# Builds the frontend and launches the server.

set -euxo pipefail

npm --version && \
	cargo --version && \
	wasm-pack --version && \
	Rscript --version \

cd frontend
NODE_OPTIONS=--openssl-legacy-provider npm install
NODE_OPTIONS=--openssl-legacy-provider npm run build
cd ..

cd server
(
export LIBRARIAN_INDEX_PATH="../frontend/dist/"
export LIBRARIAN_PORT="8186"
export LIBRARIAN_EXAMPLE_PATH="../frontend/example_inputs"
export RUST_LOG=trace,actix_http=info,mio=info,actix_server=info,actix_web=info
cargo run --bin server
)
cd ..
