#![allow(unused_unsafe)]

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod extract_comp;

pub mod io_utils {
    use gloo::file::File;
    use js_sys::Uint8Array;
    use wasm_bindgen::prelude::*;
    use web_sys::FileReaderSync;

    #[wasm_bindgen(module = "/js/exports.js")]
    extern "C" {
        pub fn update_progress();
        pub fn get_update_threshold () -> u64;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn debug(msg: &str);
    }
    use std::io::{self, BufReader, Read};

    #[derive(Debug)]
    pub struct JSFileReader {
        pos: u64,
        file_reader: FileReaderSync,
        file: File,
        progress: u64,
        thresh: u64,
    }

    #[allow(clippy::new_without_default)]
    impl JSFileReader {
        pub fn new(f: File) -> JSFileReader {
            JSFileReader {
                pos: 0,
                file_reader: FileReaderSync::new().unwrap(),
                file: f,
                progress: 0,
                thresh: get_update_threshold(),
            }
        }
    }

    impl Read for JSFileReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let sl = self.file.slice(self.pos, self.pos + buf.len() as u64);
            let arr = Uint8Array::new(&self.file_reader.read_as_array_buffer(sl.as_ref()).unwrap());
            let len = std::cmp::min(buf.len(), arr.length() as usize);

            self.progress += len as u64;
            if self.progress > self.thresh {
                update_progress();
                self.progress = 0;
            }

            arr.slice(0, len as u32).copy_to(&mut buf[..len]);

            self.pos += len as u64;
            Ok(len)
        }
    }

    // Reader is a wrapper over BufRead
    // And provides an interface over the actual reading.
    pub fn get_reader(file: File) -> BufReader<Box<dyn Read>> {
        let typ = file.raw_mime_type();
        let compressed = typ == "application/gzip" || typ == "application/x-gzip";

        fastq2comp::io_utils::compressed_reader(
            Box::new(JSFileReader::new(file)) as Box<dyn Read>,
            compressed,
        )
    }
}
