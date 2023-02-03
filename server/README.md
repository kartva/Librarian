# Server

The server runs the data processing and visualization code.

Serves the webpages in `frontend/dist` along with running backend services.
## Setup:

```bash
docker pull docker.io/desmondwillowbrook/librarian-server
docker run -dp 8186:8186 desmondwillowbrook/librarian-server
```

### Non-Docker setup
Install:
- [`npm`](https://www.npmjs.com/get-npm)
- [`Rust (with Cargo)`](https://www.rust-lang.org/) 
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- R

```bash
apt-get install -y r-base-core r-base-dev libssl-dev libcurl4-openssl-dev libxml2-dev
Rscript -e 'install.packages(c("tidyverse", "umap", "svglite", "ggrastr")) '
Rscript -e 'install.packages("remotes")'
Rscript -e 'remotes::install_github("rstudio/pins")'
```

```bash
git clone https://github.com/DesmondWillowbrook/Librarian.git
cd Librarian
```

```bash
./run.sh
```

Alternatively, in case you haven't made any changes to the frontend, you can just run the server binary without rebuilding the frontend.

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