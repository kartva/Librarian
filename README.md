<center>
<img src="frontend/static/favicon.ico" />

# Librarian 
</center>

A quality-assurance tool to sanity check FASTQ compositions and their library types.

## Setup:

```bash
docker pull docker.io/desmondwillowbrook/librarian-server
docker run -dp 8186:8186 desmondwillowbrook/librarian-server
```

## Non-Docker setup
Install:
- [`npm`](https://www.npmjs.com/get-npm)
- [`Rust (with Cargo)`](https://www.rust-lang.org/) 
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- R
```
apt-get install -y r-base-core r-base-dev libssl-dev libcurl4-openssl-dev libxml2-dev
Rscript -e 'install.packages(c("tidyverse", "umap")) '
```

```bash
git clone https://github.com/DesmondWillowbrook/Librarian-Server.git
```

```bash
cd frontend
npm install
wasm-pack build
npm run build # or npm start for interactive dev server
```

```bash
cd server
LIBRARIAN_INDEX_PATH="../frontend/dist/"
cargo run --bin server --release
```

### Batch query cli
(currently queries from localhost)
```bash
cd cli
cargo run --release -- 100000 ../frontend/example_inputs/in.fastq
```
Plots will be produced in the same directory as of input file.

### Attribution:
- `favicon.ico` sourced from [favicon.io](https://favicon.io/emoji-favicons/books) sourced from [twemoji](https://twemoji.twitter.com/), licensed under [CC BY-4](https://creativecommons.org/licenses/by/4.0/).

### Associated repositories:
- [Library Base Compositions](https://github.com/ChristelKrueger/Library_Base_Compositions)
- [Web Library Base Compositions] https://github.com/DesmondWillowbrook/Web_Library_Base_Compositions
