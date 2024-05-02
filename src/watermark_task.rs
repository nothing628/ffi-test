use anyhow::{anyhow, Result};
use image::{
    load_from_memory_with_format, DynamicImage, GenericImage, GenericImageView, ImageFormat,
    ImageResult,
};
use std::mem::transmute;

#[derive(PartialEq, Eq, Debug)]
pub enum OriginX {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
pub enum OriginY {
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct WatermarkTask {
    watermark: Option<DynamicImage>,
    target: Option<DynamicImage>,
    output: Option<DynamicImage>,
    origin_x: OriginX,
    origin_y: OriginY,
    x: u32,
    y: u32,
}

pub struct Point {
    x: u32,
    y: u32,
}

fn solve_absolute_position(
    target_img: &DynamicImage,
    watermark_img: &DynamicImage,
    origin_x: &OriginX,
    origin_y: &OriginY,
    offset: &Point,
) -> Point {
    let (target_w, target_h) = target_img.dimensions();
    let (watermark_w, watermark_h) = watermark_img.dimensions();
    let offset_x = offset.x;
    let offset_y = offset.y;
    let abs_x = if *origin_x == OriginX::Left {
        offset_x
    } else {
        target_w - watermark_w - offset_x
    };
    let abs_y = if *origin_y == OriginY::Top {
        offset_y
    } else {
        target_h - watermark_h - offset_y
    };

    Point { x: abs_x, y: abs_y }
}

impl WatermarkTask {
    pub fn new() -> Self {
        WatermarkTask {
            origin_x: OriginX::Left,
            origin_y: OriginY::Top,
            x: 0,
            y: 0,
            target: None,
            watermark: None,
            output: None,
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32, origin_x: OriginX, origin_y: OriginY) {
        self.x = x;
        self.y = y;
        self.origin_x = origin_x;
        self.origin_y = origin_y;
    }

    pub fn set_target(&mut self, target: Option<DynamicImage>) {
        self.target = target;
    }

    pub fn set_watermark(&mut self, watermark: Option<DynamicImage>) {
        self.watermark = watermark;
    }

    pub fn process(&mut self) -> Result<()> {
        let target = &mut self.target;
        let watermark = &self.watermark;
        let offset_x = self.x;
        let offset_y = self.y;
        let offset = Point { x: offset_x, y: offset_y };
        let origin_x = &self.origin_x;
        let origin_y = &self.origin_y;

        if let Some(target_img) = target {
            if let Some(watermark_img) = watermark {
                let pos = solve_absolute_position(&target_img, &watermark_img, origin_x, origin_y, &offset);
                let mut clone_target = target_img.clone();
                clone_target.copy_from(watermark_img, pos.x, pos.y)?;

                self.output = Some(clone_target);

                Ok(())
            } else {
                Err(anyhow!("Watermark is not set"))
            }
        } else {
            Err(anyhow!("Target is not set"))
        }
    }
}

fn set_watermark(
    watermark_task: &mut WatermarkTask,
    bytes: &[u8],
    format: ImageFormat,
) -> ImageResult<()> {
    let watermark = load_from_memory_with_format(bytes, format)?;
    watermark_task.set_watermark(Some(watermark));

    Ok(())
}

fn set_target(
    watermark_task: &mut WatermarkTask,
    bytes: &[u8],
    format: ImageFormat,
) -> ImageResult<()> {
    let target = load_from_memory_with_format(bytes, format)?;
    watermark_task.set_target(Some(target));

    Ok(())
}

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

