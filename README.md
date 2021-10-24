<center>
<img src="frontend/static/favicon.ico" />

# Librarian 
</center>

A quality-assurance tool to sanity check FASTQ compositions and their library types.

## Setup:

```bash
docker pull desmondwillowbrook/librarian-server
docker run -dp 8186:8186 desmondwillowbrook/librarian-server
```

## Non-Docker setup
Install:
- [`npm`](https://www.npmjs.com/get-npm)
- [`Rust (with Cargo)`](https://www.rust-lang.org/) 
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

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
cargo run --release
```

### Attribution:
- `favicon.ico` sourced from [favicon.io](https://favicon.io/emoji-favicons/books) sourced from [twemoji](https://twemoji.twitter.com/), licensed under [CC BY-4](https://creativecommons.org/licenses/by/4.0/).