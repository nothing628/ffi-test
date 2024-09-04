#[macro_export]
macro_rules! create_get_old_section_func {
    ($t:ident, $export:expr) => {
        #[wasm_bindgen]
        pub fn $t(ptr: *mut WatermarkTask) -> Result<JsValue, JsValue> {
            let watermark_task = unsafe { &mut *ptr };
            let output = watermark_task.get_old_section();

            if let Some(output_img) = output {
                let mut bytes: Vec<u8> = Vec::new();
                let mut cur = Cursor::new(&mut bytes);
                let output_bin = output_img.write_to(&mut cur, $export);

                if let Err(_) = output_bin {
                    let err_msg = serde_wasm_bindgen::to_value("Failed to write output")?;
                    return Err(err_msg);
                }

                let output_bytes = serde_wasm_bindgen::to_value(&bytes)?;
                return Ok(output_bytes);
            }

            let err_msg = serde_wasm_bindgen::to_value("Task not yet processed")?;
            return Err(err_msg);
        }
    };
}

#[macro_export]
macro_rules! create_get_output_func {
    ($t:ident, $u:ident, $v:ident, $export:expr) => {
        #[wasm_bindgen]
        pub fn $t(ptr: *mut WatermarkTask) -> Result<JsValue, JsValue> {
            let watermark_task = unsafe { &mut *ptr };
            return $u(watermark_task);
        }

        fn $u(watermark_task: &mut WatermarkTask) -> Result<JsValue, JsValue> {
            let output = watermark_task.get_output();
            let old_section = watermark_task.get_old_section();
            let mut bytes: Vec<u8> = Vec::new();
            let mut old_bytes: Vec<u8> = Vec::new();

            if let Some(output_img) = output {
                let mut cur = Cursor::new(&mut bytes);
                let output_bin = output_img.write_to(&mut cur, $export);

                if let Err(_) = output_bin {
                    let err_message = serde_wasm_bindgen::to_value("Cannot write output bytes")?;
                    return Err(err_message);
                }
            } else {
                let err_message = serde_wasm_bindgen::to_value("Task not yet processed")?;
                return Err(err_message);
            }

            if let Some(old_img) = old_section {
                let mut cur_old = Cursor::new(&mut old_bytes);
                let output_old = old_img.write_to(&mut cur_old, $export);

                if let Err(_) = output_old {
                    let err_message =
                        serde_wasm_bindgen::to_value("Cannot write old section bytes")?;
                    return Err(err_message);
                }
            } else {
                let err_message = serde_wasm_bindgen::to_value("Task not yet processed")?;
                return Err(err_message);
            }

            if let None = watermark_task.get_key() {
                let err_message = serde_wasm_bindgen::to_value("Encryption key not set")?;
                return Err(err_message);
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
                let result_values = serde_wasm_bindgen::to_value(&result)?;
                return Ok(result_values);
            }

            let err_message = serde_wasm_bindgen::to_value("Unknown error")?;
            return Err(err_message);
        }
    };
}

#[macro_export]
macro_rules! create_set_watermark_func {
    ($t:ident,$typ:expr) => {
        #[wasm_bindgen]
        pub fn $t(ptr: *mut WatermarkTask, inp_bytes: Vec<u8>) -> Result<(),JsValue> {
            let watermark_task = unsafe { &mut *ptr };

            if let Err(_) = set_watermark(watermark_task, &inp_bytes, $typ) {
                let err_message = serde_wasm_bindgen::to_value("Cannot set watermark")?;
                return Err(err_message);
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! create_set_target_func {
    ($t:ident,$typ:expr) => {
        #[wasm_bindgen]
        pub fn $t(ptr: *mut WatermarkTask, inp_bytes: Vec<u8>) -> Result<(),JsValue> {
            let watermark_task = unsafe { &mut *ptr };

            if let Err(_) = set_target(watermark_task, &inp_bytes, $typ) {
                let err_message = serde_wasm_bindgen::to_value("Cannot set target")?;
                return Err(err_message);
            }
            Ok(())
        }
    };
}
