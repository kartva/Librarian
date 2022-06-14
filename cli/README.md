# Librarian Query CLI

Set env variable `RUST_LOG` to `trace`, `debug`, `info`, `warn`, `error` to filter logs. (Default is `info` level)

## Testing with local server
```bash
(
export LIBRARIAN_API_URL=http://127.0.0.1:8186/api/plot_comp
export RUST_LOG=trace
cargo run --release -- ../frontend/example_inputs/* --emit-compositions
)
```