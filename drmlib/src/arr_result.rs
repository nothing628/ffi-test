use std::mem::{forget, transmute};
use wasm_bindgen::prelude::*;

pub struct ArrResult {
    pub arr: Vec<u8>,
}

#[wasm_bindgen]
pub fn create_arr_result() -> *mut ArrResult {
    let arr_result = ArrResult {
        arr: Vec::new(),
    };
    let ptr = unsafe { transmute(Box::new(arr_result)) };

    ptr
}

#[wasm_bindgen]
pub fn len_arr_result(ptr: *mut ArrResult) -> usize {
    let arr_result = unsafe { & *ptr };
    arr_result.arr.len()
}

#[wasm_bindgen]
pub fn read_arr_result(ptr: *mut ArrResult, len: usize) -> *const u8 {
    let arr_result = unsafe { & *ptr };
    let cpy = arr_result.arr[0..len].to_vec();
    let ptr = cpy.as_ptr();

    forget(cpy);
    ptr
}

#[wasm_bindgen]
pub fn destroy_arr_result(ptr: *mut ArrResult) {
    let _counter: Box<ArrResult> = unsafe{ transmute(ptr) };
    // Drop
}
