use image::ImageFormat;
use std::io::Cursor;
use std::mem::transmute;

use drmcore::file_joiner::{join_jpeg, join_webp};
use drmcore::watermark_task::{set_target, set_watermark, OriginX, OriginY, WatermarkTask};
use wasm_bindgen::prelude::*;

use crate::arr_result::ArrResult;
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
pub fn set_key(ptr: *mut WatermarkTask, byts_ptr: *const u8, byts_len: usize) -> u32 {
    let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let watermark_task = unsafe { &mut *ptr };

    if let Err(_) = watermark_task.set_key(byts) {
        return 1;
    }

    0
}

#[wasm_bindgen]
pub fn destroy_watermarktask(ptr: *mut WatermarkTask) {
    let _counter: Box<WatermarkTask> = unsafe { transmute(ptr) };
    // Drop
}

#[wasm_bindgen]
pub fn process_watermark(ptr: *mut WatermarkTask) -> u32 {
    let watermark_task = unsafe { &mut *ptr };
    let output = watermark_task.process();

    if let Err(_) = output {
        return 1;
    }
    0
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
        let mut arr_result = ArrResult { arr: Vec::new() };
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

        get_output_webp_native(&mut watermark_task, &mut arr_result);
    }
}
