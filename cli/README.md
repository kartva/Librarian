# Librarian Query CLI

Queries the Babraham server (or your own!) with supplied FASTQ files and downloads plots.

```
Librarian CLI 1.1.0
A tool to predict the sequencing library type from the base composition of a supplied FastQ file. Uncompresses .gz files
when reading.

USAGE:
    librarian [FLAGS] [OPTIONS] <input>...

FLAGS:
    -h, --help       
            Prints help information

    -l, --local      
            Run all processing locally, replacing the need for a server. Requires Rscript and other dependencies to be
            installed, along with the `scripts` folder.
            
            This cannot be set together with `api`.
    -q, --quiet      
            Suppresses all output except errors

    -V, --version    
            Prints version information


OPTIONS:
        --api <api>          
            Specifies query URL to send prediction request to. Defaults to Babraham Bioinformatic's server. Passed
            argument is given precedence over environment variable.
            
            This cannot be set together with --local. [env: LIBRARIAN_API_URL=]  [default:
            https://www.bioinformatics.babraham.ac.uk/librarian/api/plot_comp]
    -o, --prefix <prefix>    
            Prefix to append to output files (eg. `output_dir/` or `name_`) Note that this can be used to set an output
            directory [default: librarian_]

ARGS:
    <input>...    
            List of input files
```

## Installation

CLI binaries can be found in the [Github Releases](https://github.com/DesmondWillowbrook/Librarian/releases) section of the repository.

The released binaries are statically linked with `musl`, so there shouldn't be too much in the way of system requirements for the CLI except running Linux.

Running with the `--local` option **requires additional dependencies** to be installed. Refer to the [server README's Runtime Dependencies section](../server/README.md#runtime-dependencies) for a list of the other dependencies. 

## Install from source
Requires a recent version of the [Rust](https://rust-lang.org) toolchain. 
```bash
cargo install --git "https://github.com/DesmondWillowbrook/Librarian/" cli
```

## Querying another server
```bash
librarian example.fastq.gz --api http://127.0.0.1:8186/api/plot_comp
```

To debug the client:
```bash
(
export LIBRARIAN_API_URL=http://127.0.0.1:8186/api/plot_comp
export RUST_LOG=trace # other values are `debug`, `info`, `warn` and `error` - default is `info`
cargo run --release --bin librarian -- ../frontend/example_inputs/example_inputs/*
)
```