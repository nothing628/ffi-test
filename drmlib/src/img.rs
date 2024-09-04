use std::mem::transmute;
use drmcore::img::get_section_jpeg as img_get_section_jpeg;
use drmcore::img::get_section_webp as img_get_section_webp;
use crate::arr_result::ArrResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_section_webp(
    byts_ptr: *const u8,
    byts_len: usize,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> *mut ArrResult {
    let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let result = img_get_section_webp(byts, x, y, w, h);
    let arr_result = ArrResult {
        arr: result.clone(),
    };
    let ptr = unsafe { transmute(Box::new(arr_result)) };

    ptr
}

#[wasm_bindgen]
pub fn get_section_jpeg(
    byts_ptr: *const u8,
    byts_len: usize,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> *mut ArrResult {
    let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let result = img_get_section_jpeg(byts, x, y, w, h);
    let arr_result = ArrResult {
        arr: result.clone(),
    };
    let ptr = unsafe { transmute(Box::new(arr_result)) };

    ptr
}
