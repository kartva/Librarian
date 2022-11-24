# fastq2comp

Library to read base compositions from FASTQ files. Given a valid file, produces a the percentages of each base per position.

For an overview of usage, check out the [example program](examples/extract-comp/main.rs). You can also test the library through the example by replacing `in.fastq` with your file and looking at `out.json` (run ``cargo run --example extract-comp -- 100 "examples/extract-comp/in.fastq"``)

You can view the crate documentation via `cargo doc --open`.