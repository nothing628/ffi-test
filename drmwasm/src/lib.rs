mod utils;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add(x: u32) -> u32 {
    x + 2
}

#[wasm_bindgen]
pub struct ReplacementImage {
    pub x: u32,
    pub y: u32,
    real_img: Vec<u8>,
}

#[wasm_bindgen]
pub fn get_replacement_img(inp_bytes: Vec<u8>) -> Vec<u8> {
    Vec::from([1,2,4])
}
