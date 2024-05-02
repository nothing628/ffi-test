mod watermark_task;
mod img;

use std::mem::{forget, transmute};

use crate::img::get_section_webp as img_get_section_webp;
use crate::img::get_section_jpeg as img_get_section_jpeg;

pub struct ArrResult {
    arr: Vec<u8>,
}

#[no_mangle]
pub extern "C" fn add(a: u32) -> u32 {
    a + 1222
}

#[no_mangle]
pub extern "C" fn add_array(byts_ptr: *const u32, byts_len: usize) -> u32 {
    let bys = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let mut result = 0u32;

    for by in bys {
        result += *by;
    }

    result
}

#[no_mangle]
pub extern "C" fn ret_arr() -> *mut u32 {
    let mut test = vec![111, 222, 333, 444, 555, 666, 777, 888, 999, 0];
    let ptr = test.as_mut_ptr();

    forget(test); // so that it is not destructed at the end of the scope

    ptr
}

#[no_mangle]
pub extern "C" fn get_section_webp(
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

#[no_mangle]
pub extern "C" fn get_section_jpeg(
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

#[no_mangle]
pub extern fn len_arr_result(ptr: *mut ArrResult) -> usize {
    let arr_result = unsafe { & *ptr };
    arr_result.arr.len()
}

#[no_mangle]
pub extern fn read_arr_result(ptr: *mut ArrResult, len: usize) -> *const u8 {
    let arr_result = unsafe { & *ptr };
    let cpy = arr_result.arr[0..len].to_vec();
    let ptr = cpy.as_ptr();

    forget(cpy);
    ptr
}

#[no_mangle]
pub extern fn destroy_arr_result(ptr: *mut ArrResult) {
    let _counter: Box<ArrResult> = unsafe{ transmute(ptr) };
    // Drop
}
