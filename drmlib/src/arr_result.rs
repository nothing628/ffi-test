use std::mem::{forget, transmute};

pub struct ArrResult {
    pub arr: Vec<u8>,
}

#[no_mangle]
pub extern fn create_arr_result() -> *mut ArrResult {
    let arr_result = ArrResult {
        arr: Vec::new(),
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
