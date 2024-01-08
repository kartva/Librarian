# Server
Serves the webpages in `frontend/dist` along with running backend services.

## Installation

### Build dependencies
- [`npm`](https://www.npmjs.com/get-npm)
- [`Rust (with Cargo)`](https://www.rust-lang.org/) 
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

### Runtime dependencies
The `scripts` folder included with the binary should be in the same directory as the binary. `Rscript` should be present in $PATH.

```bash
apt-get install -y r-base-core r-base-dev libssl-dev libcurl4-openssl-dev libxml2-dev
Rscript -e 'install.packages(c("tidyverse", "umap", "ggrastr", "pins", "rmarkdown"))'
```

### Building from source

```bash
git clone https://github.com/DesmondWillowbrook/Librarian.git
cd Librarian
./run-server.sh
```

Alternatively, in case you haven't made any changes to the frontend, you can just run the server binary without rebuilding the frontend website.

```
cd server
cargo run --release
```

## Environment Variables:
- `LIBRARIAN_PORT` (defaults to 8186): port to listen to.
- `LIBRARIAN_INDEX_PATH` (defaults to `../frontend/dist`): path to directory to serve (the one with `index.html`).
- `LIBRARIAN_EXAMPLE_PATH` (defaults to `../frontend/example_inputs`): path to example input files.

## Debugging
```
RUST_LOG=trace cargo run --release
```

**NOTE:** Setting `RUST_LOG` to `trace` will cause the server to not delete temporary directories after use. This is useful for debugging purposes.