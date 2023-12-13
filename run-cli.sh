#!/bin/bash
RUST_LOG=trace cargo run --bin librarian -- frontend/example_inputs/example_inputs/ATAC.example.fastq "$@"