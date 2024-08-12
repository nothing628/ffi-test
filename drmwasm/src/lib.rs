mod utils;

use drmcore::file_splitter::{split_jpeg, split_webp};
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
pub fn get_replacement_jpeg(inp_bytes: Vec<u8>, enc_key: Vec<u8>) -> Result<JsValue, JsValue> {
    let enc_key_slice = enc_key.get(0..32);

    if let None = enc_key_slice {
        let err_data = serde_wasm_bindgen::to_value("Enc key must 32 bytes")?;
        return Err(err_data);
    }

    let enc_key_slice = enc_key_slice.unwrap();
    let fixed_enc_key: &[u8;32] = enc_key_slice.try_into().unwrap();
    let split_result = split_jpeg(&inp_bytes, fixed_enc_key);

    match split_result {
        Ok(split_data) => {
            let replacement = ReplacementImage {
                real_img: split_data.old_section_img,
                x: split_data.position.x,
                y: split_data.position.y,
                height: split_data.dimension.height,
                width: split_data.dimension.width,
            };

            return Ok(serde_wasm_bindgen::to_value(&replacement)?);
        }
        Err(err) => {
            let err_data = serde_wasm_bindgen::to_value(&err.to_string())?;
            return Err(err_data);
        },
    }
}

#[wasm_bindgen]
pub fn get_replacement_webp(inp_bytes: Vec<u8>, enc_key: Vec<u8>) -> Result<JsValue, JsValue> {
    let enc_key_slice = enc_key.get(0..32);

    if let None = enc_key_slice {
        let err_data = serde_wasm_bindgen::to_value("Enc key must 32 bytes")?;
        return Err(err_data);
    }

    let enc_key_slice = enc_key_slice.unwrap();
    let fixed_enc_key: &[u8;32] = enc_key_slice.try_into().unwrap();
    let split_result = split_webp(&inp_bytes, fixed_enc_key);

    match split_result {
        Ok(split_data) => {
            let replacement = ReplacementImage {
                real_img: split_data.old_section_img,
                x: split_data.position.x,
                y: split_data.position.y,
                height: split_data.dimension.height,
                width: split_data.dimension.width,
            };

            return Ok(serde_wasm_bindgen::to_value(&replacement)?);
        }
        Err(err) => {
            let err_data = serde_wasm_bindgen::to_value(&err.to_string())?;
            return Err(err_data);
        },
    }
}
