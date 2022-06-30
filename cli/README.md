# Librarian Query CLI

Queries the Babraham server with supplied FASTQ files and downloads plots.

```
Librarian CLI 0.1.0
Utility CLI to query the Librarian server for plotting base compositions. Uncompresses .gz files when reading

USAGE:
    librarian [FLAGS] [OPTIONS] <input>...

FLAGS:
        --emit-compositions    Intended for debugging: shows the base compositions derived from files
    -h, --help                 Prints help information
    -V, --version              Prints version information

OPTIONS:
    -o, --output <outdir>    Output path

ARGS:
    <input>...    List of input files
```

## Install from source
Requires a recent version of the [Rust](https://rust-lang.org) toolchain. 
```bash
cargo install --git "https://github.com/DesmondWillowbrook/Librarian/" cli
```

## Querying another server
```bash
(
export LIBRARIAN_API_URL=http://127.0.0.1:8186/api/plot_comp
export RUST_LOG=trace # other values are `debug`, `info`, `warn` and `error` - default is `info`
./librarian -- ../frontend/example_inputs/* --emit-compositions
)
```