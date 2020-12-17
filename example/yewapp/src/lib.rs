#![feature(box_patterns)]
#![feature(box_syntax)]


pub mod dialog;

use wasm_bindgen::prelude::*;
use yew::App;



#[wasm_bindgen(start)]
pub fn run_app() {
    App::<dialog::DAOApp>::new().mount_to_body();
}



#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log2(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = window.ethereum, js_name = request)]
    fn request(m: &str);
}
