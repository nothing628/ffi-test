#[macro_export]
macro_rules! create_get_old_section_func {
    ($t:ident, $export:expr) => {
        #[wasm_bindgen]
        pub fn $t(ptr: *mut WatermarkTask, target: *mut ArrResult) -> u32 {
            let watermark_task = unsafe { &mut *ptr };
            let target_arr = unsafe { &mut *target };
            let output = watermark_task.get_old_section();
        
            if let Some(output_img) = output {
                let mut bytes: Vec<u8> = Vec::new();
                let mut cur = Cursor::new(&mut bytes);
                let output_bin = output_img.write_to(&mut cur, $export);
        
                if let Err(_) = output_bin {
                    return 2;
                }
        
                target_arr.arr = bytes;
        
                return 0;
            }
        
            1
        }
    };
}