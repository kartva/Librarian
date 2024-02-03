use fastq2comp::extract_comp::{run_json, FASTQReader, SampleArgs};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{io_utils::create_reader, utils::set_panic_hook};

// Entry points here
#[wasm_bindgen]
pub fn run_json_exported(f: web_sys::File) -> String {
    set_panic_hook();

    let fastq_reader = FASTQReader::new(SampleArgs::default(), create_reader(f.into()));
    run_json(fastq_reader)
}
