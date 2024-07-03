mod utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: u32) -> u32 {
    x + 2
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ReplacementImage {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    real_img: Vec<u8>,
}

#[wasm_bindgen]
pub fn get_replacement_img(inp_bytes: Vec<u8>) -> Result<JsValue, JsValue>{
    let replacement = ReplacementImage {
        real_img: Vec::from([29, 124, 55]),
        x: 12,
        y: 41,
        height: 123,
        width: 400,
    };

    Ok(serde_wasm_bindgen::to_value(&replacement)?)
}
