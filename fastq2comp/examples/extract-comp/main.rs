/**
 * Run with the command:
 * cargo run --example extract-comp -- 100 "examples/extract-comp/in.fastq"
 */

use fastq2comp::extract_comp::FASTQReader;
use fastq2comp::extract_comp::{run_json, SampleArgs};
use fastq2comp::io_utils;

use clap::Parser;

use std::fs::File;
use std::{io::Write, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Target read count
    target_read_count: u64,

    /// File to extract reads from.
    file: String // this should technically be a PathBuf... but non-UTF paths
}

/// This example extracts the base composition of a file
/// and prints it JSON format.

fn main() {
    let cli = Cli::parse();

    let path = cli.file;
    let f = File::open(path).unwrap();
    let mut reader = io_utils::compressed_reader(f, false);

    let result = run_json(FASTQReader::new(SampleArgs {target_read_count: cli.target_read_count, ..SampleArgs::default()}, &mut reader));

    let mut file = match File::create(&PathBuf::from("examples/extract-comp/out.json")) {
        Err(why) => panic!("Couldn't open output JSON file: {}", why),
        Ok(file) => file,
    };

    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to output JSON file: {}", why),
        Ok(_) => println!("successfully wrote to output JSON file, read {}", result),
    }
}
