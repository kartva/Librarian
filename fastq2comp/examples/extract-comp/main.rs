use fastq2comp::extract_comp::{SampleArgs, run_json};
use fastq2comp::io_utils;
use fastq2comp::extract_comp::FASTQReader;

use std::{path::PathBuf, io::Write};
use std::fs::File;

/// This example extracts the base composition of a file
/// and prints it JSON format.

fn main() {
    let path = "examples/extract-comp/in.fastq";
    let f = File::open(path).unwrap();
    let mut reader = io_utils::compressed_reader(f, false);

    let result = run_json(FASTQReader::new(SampleArgs::default(), &mut reader));

    let mut file = match File::create(&PathBuf::from("examples/extract-comp/out.json")) {
        Err(why) => panic!("Couldn't open output JSON file: {}", why),
        Ok(file) => file,
    };

    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to output JSON file: {}", why),
        Ok(_) => println!("successfully wrote to output JSON file, read {}", result),
    }
}