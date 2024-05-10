use std::mem::forget;

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
