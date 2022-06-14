### Run server
```
cd server
cargo run --release
```

### Environment Variables:
- `LIBRARIAN_PORT` (defaults to 8186): port to listen to.
- `LIBRARIAN_INDEX_PATH` (defaults to `../frontend/dist`): path to directory to serve (the one with `index.html`).
- `LIBRARIAN_INPUT_PATH` (defaults to `../frontend/example_inputs`): path to example input files.

### Debugging
```
RUST_LOG=trace cargo run --release
```