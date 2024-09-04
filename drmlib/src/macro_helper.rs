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

#[macro_export]
macro_rules! create_get_output_func {
    ($t:ident, $u:ident, $v:ident, $export:expr) => {
        #[wasm_bindgen]
        pub fn $t(ptr: *mut WatermarkTask, target: *mut ArrResult) -> u32 {
            let watermark_task = unsafe { &mut *ptr };
            let target_arr = unsafe { &mut *target };
            return $u(watermark_task, target_arr);
        }

        fn $u(watermark_task: &mut WatermarkTask, target_arr: &mut ArrResult) -> u32 {
            let output = watermark_task.get_output();
            let old_section = watermark_task.get_old_section();
            let mut bytes: Vec<u8> = Vec::new();
            let mut old_bytes: Vec<u8> = Vec::new();

            if let Some(output_img) = output {
                let mut cur = Cursor::new(&mut bytes);
                let output_bin = output_img.write_to(&mut cur, $export);

                if let Err(_) = output_bin {
                    return 2;
                }
            } else {
                return 1;
            }

            if let Some(old_img) = old_section {
                let mut cur_old = Cursor::new(&mut old_bytes);
                let output_old = old_img.write_to(&mut cur_old, $export);

                if let Err(_) = output_old {
                    return 3;
                }
            } else {
                return 1;
            }

            if let None = watermark_task.get_key() {
                return 4;
            }

            let watermark_pos: [u8; 8] = watermark_task
                .get_absolute_watermark_position()
                .unwrap()
                .into();
            let watermark_dim: [u8; 8] = watermark_task.get_watermark_dimension().unwrap().into();
            old_bytes.extend(watermark_pos);
            old_bytes.extend(watermark_dim);

            let enc_key = watermark_task.get_key().unwrap();
            let join_result = $v(&bytes, &old_bytes, &enc_key);
            if let Ok(result) = join_result {
                target_arr.arr = result;

                return 0;
            }

            5
        }
    };
}

#[macro_export]
macro_rules! create_set_watermark_func {
    ($t:ident,$typ:expr) => {
        #[wasm_bindgen]
        pub fn $t(ptr: *mut WatermarkTask, byts_ptr: *const u8, byts_len: usize) -> u32 {
            let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
            let watermark_task = unsafe { &mut *ptr };

            if let Err(_) = set_watermark(watermark_task, byts, $typ) {
                return 1;
            }
            0
        }
    };
}

#[macro_export]
macro_rules! create_set_target_func {
    ($t:ident,$typ:expr) => {
        #[wasm_bindgen]
        pub fn $t(
            ptr: *mut WatermarkTask,
            byts_ptr: *const u8,
            byts_len: usize,
        ) -> u32 {
            let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
            let watermark_task = unsafe { &mut *ptr };

            if let Err(_) = set_target(watermark_task, byts, $typ) {
                return 1;
            }
            0
        }
    };
}
