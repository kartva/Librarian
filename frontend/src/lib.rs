#![allow(unused_unsafe)]

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod extract_comp;

pub mod io_utils {    
    use js_sys::Uint8Array;
    use wasm_bindgen::prelude::*;
    use gloo::file::File;
    use web_sys::FileReaderSync;
        
    #[wasm_bindgen(module = "/js/exports.js")]
    //#[link(wasm_import_module = "/web_library_base_compositions.js")]
    extern "C" {
        pub fn get_file() -> web_sys::File; 
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn debug(msg: &str);
    }
    use std::io::{self, BufRead, BufReader, Read};


    // Credit to: mstange on GitHub
    // See: https://github.com/rustwasm/wasm-bindgen/issues/1079#issuecomment-508577627
    #[derive(Debug)]
    pub struct WasmMemBuffer {pos: u64, file: File}
    
    impl WasmMemBuffer {
        pub fn new() -> WasmMemBuffer {
            WasmMemBuffer {file: File::from(unsafe {get_file()}), pos: 0}
        }
    }

    impl Read for WasmMemBuffer {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let fr = FileReaderSync::new().unwrap();
            let sl = self.file.slice(self.pos, self.pos + buf.len() as u64);
            let arr = Uint8Array::new(unsafe {&fr.read_as_array_buffer(sl.as_ref()).unwrap()});
            let len = std::cmp::min(buf.len(), arr.length() as usize);

            arr.slice(0, len as u32).copy_to(&mut buf[..len]);

            self.pos += len as u64;
            Ok(len as usize)
        }
    }

    use flate2::read::GzDecoder;
    // Reader is a wrapper over BufRead
    // And provides an interface over the actual reading.
    pub fn get_reader(compressed: bool) -> Box<dyn BufRead> {
        Box::new(BufReader::new(    
            if compressed {
                Box::new(GzDecoder::new(WasmMemBuffer::new()))
            } else {Box::new(WasmMemBuffer::new()) as Box<dyn Read>}
        ))
    }
}