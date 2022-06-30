use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/js/exports.js")]
extern "C" {
    pub fn set_error(msg: String);
}

pub fn set_panic_hook() {
    fn hook_impl(info: &panic::PanicInfo) {
        let msg = info.to_string();
        set_error(msg);
    }

    panic::set_hook(Box::new(hook_impl));
}
