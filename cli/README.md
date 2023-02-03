# Librarian Query CLI

Queries the Babraham server with supplied FASTQ files and downloads plots.

```
Librarian CLI 1.0.3
A tool to predict the sequencing library type from the base composition of a supplied FastQ file. Uncompresses .gz files
when reading.

USAGE:
    librarian [OPTIONS] <input>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <outdir>    Output path

ARGS:
    <input>...    List of input files
```

## Installation

CLI binaries can be found in the [Github Releases](https://github.com/DesmondWillowbrook/Librarian/releases) section of the repository.

The released binaries are statically linked with `musl`, so there shouldn't be too much in the way of system requirements for the CLI except running Linux.

The server has more complex installation requirements, which can be found on the [server README](../server/README.md). 

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
cargo run --release --bin librarian -- PATH_TO_INPUT
)
```

For example, to pass the example files, first uncompress with `unzip example_inputs.zip` in `../frontend/example_inputs/` and then run 
```bash
(
export LIBRARIAN_API_URL=http://127.0.0.1:8186/api/plot_comp
export RUST_LOG=trace # other values are `debug`, `info`, `warn` and `error` - default is `info`
cargo run --release --bin librarian -- ../frontend/example_inputs/example_inputs/*
)
```