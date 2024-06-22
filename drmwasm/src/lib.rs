mod utils;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add(x: u32) -> u32 {
    x + 2
}

#[wasm_bindgen]
pub struct ReplacementImage {
    x: u32,
    y: u32,
    real_img: Vec<u8>,
}

#[wasm_bindgen]
pub fn get_replacement_img(inp_bytes: Vec<u8>) -> ReplacementImage {
    ReplacementImage {
        x: 12,
        y: 32,
        real_img: Vec::new(),
    }
}
