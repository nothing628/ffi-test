use image::ImageFormat;
use std::io::Cursor;
use std::mem::transmute;

use drmcore::file_joiner::{join_jpeg, join_webp};
use drmcore::watermark_task::{set_target, set_watermark, OriginX, OriginY, WatermarkTask};
use wasm_bindgen::prelude::*;

use crate::{
    create_get_old_section_func, create_get_output_func, create_set_target_func,
    create_set_watermark_func,
};

#[wasm_bindgen]
pub fn create_watermarktask() -> *mut WatermarkTask {
    let watermark_task = WatermarkTask::new();
    let ptr = unsafe { transmute(Box::new(watermark_task)) };

    ptr
}

#[wasm_bindgen]
pub fn set_position_watermark(ptr: *mut WatermarkTask, x: u32, y: u32, origin_x: u8, origin_y: u8) {
    let watermark_task = unsafe { &mut *ptr };
    let real_origin_x = if origin_x == 0 {
        OriginX::Left
    } else {
        OriginX::Right
    };
    let real_origin_y = if origin_y == 0 {
        OriginY::Top
    } else {
        OriginY::Bottom
    };
    watermark_task.set_position(x, y, real_origin_x, real_origin_y);
}

#[wasm_bindgen]
pub fn set_key(ptr: *mut WatermarkTask, key: Vec<u8>) -> Result<(),JsValue> {
    let watermark_task = unsafe { &mut *ptr };

    if let Err(_) = watermark_task.set_key(&key) {
        let err_msg = serde_wasm_bindgen::to_value("Cannot set encryption key")?;
        return Err(err_msg);
    }
    Ok(())
}

#[wasm_bindgen]
pub fn destroy_watermarktask(ptr: *mut WatermarkTask) {
    let _counter: Box<WatermarkTask> = unsafe { transmute(ptr) };
    // Drop
}

#[wasm_bindgen]
pub fn process_watermark(ptr: *mut WatermarkTask) -> Result<(),JsValue> {
    let watermark_task = unsafe { &mut *ptr };
    let output = watermark_task.process();

    if let Err(_) = output {
        let err_msg = serde_wasm_bindgen::to_value("Cannot process")?;
        return Err(err_msg);
    }
    Ok(())
}

create_set_target_func! {set_target_webp,ImageFormat::WebP}
create_set_target_func! {set_target_jpeg,ImageFormat::Jpeg}
create_set_watermark_func! {set_watermark_webp,ImageFormat::WebP}
create_set_watermark_func! {set_watermark_jpeg,ImageFormat::Jpeg}
create_get_old_section_func! {get_old_section_jpeg,ImageFormat::Jpeg}
create_get_old_section_func! {get_old_section_webp,ImageFormat::WebP}
create_get_output_func! {get_output_jpeg,get_output_jpeg_native,join_jpeg,ImageFormat::Jpeg}
create_get_output_func! {get_output_webp,get_output_webp_native,join_webp,ImageFormat::WebP}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webp_watermark_task() {
        let mut watermark_task = WatermarkTask::new();
        let watermark = image::open("../watermark.webp").unwrap();
        let img = image::open("../test.webp").unwrap();

        watermark_task.set_target(Some(img));
        watermark_task.set_watermark(Some(watermark));
        watermark_task.set_position(40, 40, OriginX::Right, OriginY::Bottom);
        let process_result = watermark_task.process();

        match process_result {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        }

        let output = get_output_webp_native(&mut watermark_task);
    }
}
