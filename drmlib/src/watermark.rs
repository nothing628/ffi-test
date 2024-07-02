use image::ImageFormat;
use std::io::Cursor;
use std::mem::transmute;

use drmcore::file_joiner::{join_jpeg, join_webp};
use drmcore::watermark_task::{set_target, set_watermark, OriginX, OriginY, WatermarkTask};

use crate::arr_result::ArrResult;

#[no_mangle]
pub extern "C" fn create_watermarktask() -> *mut WatermarkTask {
    let watermark_task = WatermarkTask::new();
    let ptr = unsafe { transmute(Box::new(watermark_task)) };

    ptr
}

#[no_mangle]
pub extern "C" fn set_position_watermark(
    ptr: *mut WatermarkTask,
    x: u32,
    y: u32,
    origin_x: u8,
    origin_y: u8,
) {
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

#[no_mangle]
pub extern "C" fn set_target_webp(
    ptr: *mut WatermarkTask,
    byts_ptr: *const u8,
    byts_len: usize,
) -> u32 {
    let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let watermark_task = unsafe { &mut *ptr };

    if let Err(_) = set_target(watermark_task, byts, ImageFormat::WebP) {
        return 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn set_target_jpeg(
    ptr: *mut WatermarkTask,
    byts_ptr: *const u8,
    byts_len: usize,
) -> u32 {
    let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let watermark_task = unsafe { &mut *ptr };

    if let Err(_) = set_target(watermark_task, byts, ImageFormat::Jpeg) {
        return 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn set_watermark_webp(
    ptr: *mut WatermarkTask,
    byts_ptr: *const u8,
    byts_len: usize,
) -> u32 {
    let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let watermark_task = unsafe { &mut *ptr };

    if let Err(_) = set_watermark(watermark_task, byts, ImageFormat::WebP) {
        return 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn set_watermark_jpeg(
    ptr: *mut WatermarkTask,
    byts_ptr: *const u8,
    byts_len: usize,
) -> u32 {
    let byts = unsafe { std::slice::from_raw_parts(byts_ptr, byts_len) };
    let watermark_task = unsafe { &mut *ptr };

    if let Err(_) = set_watermark(watermark_task, byts, ImageFormat::Jpeg) {
        return 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn destroy_watermarktask(ptr: *mut WatermarkTask) {
    let _counter: Box<WatermarkTask> = unsafe { transmute(ptr) };
    // Drop
}

#[no_mangle]
pub extern "C" fn process_watermark(ptr: *mut WatermarkTask) -> u32 {
    let watermark_task = unsafe { &mut *ptr };
    let output = watermark_task.process();

    if let Err(_) = output {
        return 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn get_old_section_webp(ptr: *mut WatermarkTask, target: *mut ArrResult) -> u32 {
    let watermark_task = unsafe { &mut *ptr };
    let target_arr = unsafe { &mut *target };
    let output = watermark_task.get_old_section();

    if let Some(output_img) = output {
        let mut bytes: Vec<u8> = Vec::new();
        let mut cur = Cursor::new(&mut bytes);
        let output_bin = output_img.write_to(&mut cur, ImageFormat::WebP);

        if let Err(_) = output_bin {
            return 2;
        }

        target_arr.arr = bytes;

        return 0;
    }

    1
}

#[no_mangle]
pub extern "C" fn get_old_section_jpeg(ptr: *mut WatermarkTask, target: *mut ArrResult) -> u32 {
    let watermark_task = unsafe { &mut *ptr };
    let target_arr = unsafe { &mut *target };
    let output = watermark_task.get_old_section();

    if let Some(output_img) = output {
        let mut bytes: Vec<u8> = Vec::new();
        let mut cur = Cursor::new(&mut bytes);
        let output_bin = output_img.write_to(&mut cur, ImageFormat::Jpeg);

        if let Err(_) = output_bin {
            return 2;
        }

        target_arr.arr = bytes;

        return 0;
    }

    1
}

#[no_mangle]
pub extern "C" fn get_output_webp(ptr: *mut WatermarkTask, target: *mut ArrResult) -> u32 {
    let watermark_task = unsafe { &mut *ptr };
    let target_arr = unsafe { &mut *target };
    return get_output_webp_native(watermark_task, target_arr);
}

#[no_mangle]
pub extern "C" fn get_output_jpeg(ptr: *mut WatermarkTask, target: *mut ArrResult) -> u32 {
    let watermark_task = unsafe { &mut *ptr };
    let target_arr = unsafe { &mut *target };
    let output = watermark_task.get_output();
    let old_section = watermark_task.get_old_section();
    let mut bytes: Vec<u8> = Vec::new();
    let mut old_bytes: Vec<u8> = Vec::new();

    if let Some(output_img) = output {
        let mut cur = Cursor::new(&mut bytes);
        let output_bin = output_img.write_to(&mut cur, ImageFormat::Jpeg);

        if let Err(_) = output_bin {
            return 2;
        }
    } else {
        return 1;
    }

    if let Some(old_img) = old_section {
        let mut cur_old = Cursor::new(&mut old_bytes);
        let output_old = old_img.write_to(&mut cur_old, ImageFormat::Jpeg);

        if let Err(_) = output_old {
            return 3;
        }
    } else {
        return 1;
    }

    let watermark_pos: [u8;8] = watermark_task.get_absolute_watermark_position().unwrap().into();
    let watermark_dim: [u8;8] = watermark_task.get_watermark_dimension().unwrap().into();
    old_bytes.extend(watermark_pos);
    old_bytes.extend(watermark_dim);

    let join_result = join_jpeg(&bytes, &old_bytes);
    if let Ok(result) = join_result {
        target_arr.arr = result;

        return 0;
    }

    4
}

fn get_output_webp_native(watermark_task: &mut WatermarkTask, target_arr: &mut ArrResult) -> u32 {
    let output = watermark_task.get_output();
    let old_section = watermark_task.get_old_section();
    let mut bytes: Vec<u8> = Vec::new();
    let mut old_bytes: Vec<u8> = Vec::new();

    if let Some(output_img) = output {
        let mut cur = Cursor::new(&mut bytes);
        let output_bin = output_img.write_to(&mut cur, ImageFormat::WebP);

        if let Err(_) = output_bin {
            return 2;
        }
    } else {
        return 1;
    }

    if let Some(old_img) = old_section {
        let mut cur_old = Cursor::new(&mut old_bytes);
        let output_old = old_img.write_to(&mut cur_old, ImageFormat::WebP);

        if let Err(_) = output_old {
            return 3;
        }
    } else {
        return 1;
    }

    let watermark_pos: [u8;8] = watermark_task.get_absolute_watermark_position().unwrap().into();
    let watermark_dim: [u8;8] = watermark_task.get_watermark_dimension().unwrap().into();
    old_bytes.extend(watermark_pos);
    old_bytes.extend(watermark_dim);

    let join_result = join_webp(&bytes, &old_bytes);
    if let Ok(result) = join_result {
        target_arr.arr = result;

        return 0;
    }

    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webp_watermark_task() {
        let mut watermark_task = WatermarkTask::new();
        let mut arr_result = ArrResult {
            arr: Vec::new(),
        };
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
