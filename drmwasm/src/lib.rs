mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: u32) -> u32 {
    x + 2
}
