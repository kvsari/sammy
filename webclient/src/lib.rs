//! Wasm client frontend for the `sammy/webserver`.
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;

use wasm_bindgen::prelude::*;

mod if_feature;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hlloe!");
}
