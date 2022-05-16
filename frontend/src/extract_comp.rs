use fastq2comp::extract_comp::{FASTQReader, SampleArgs, run_json};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{io_utils::get_reader, utils::set_panic_hook};


// Entry points here
#[wasm_bindgen]
pub fn run_json_exported (compressed: bool) -> String {
    set_panic_hook();

    let fastq_reader = FASTQReader::new(SampleArgs::default(), get_reader(compressed));
    run_json (fastq_reader)
}