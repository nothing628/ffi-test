use image::{load_from_memory_with_format, DynamicImage, GenericImage, ImageFormat, ImageResult};
use std::io::Cursor;

pub fn get_section_webp(img: &[u8], x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    let img = load_from_memory_with_format(img, ImageFormat::WebP).unwrap();
    let crop_img = img.crop_imm(x, y, w, h);
    let mut bytes: Vec<u8> = Vec::new();
    let mut cur = Cursor::new(&mut bytes);
    crop_img.write_to(&mut cur, ImageFormat::WebP).unwrap();

    bytes
}

pub fn get_section_jpeg(img: &[u8], x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    let img = load_from_memory_with_format(img, ImageFormat::Jpeg).unwrap();
    let crop_img = img.crop_imm(x, y, w, h);
    let mut bytes: Vec<u8> = Vec::new();
    let mut cur = Cursor::new(&mut bytes);
    crop_img
        .write_to(&mut cur, image::ImageFormat::Jpeg)
        .unwrap();

    bytes
}

pub fn put_watermark(
    target: &mut DynamicImage,
    watermark: &DynamicImage,
    x: u32,
    y: u32,
) -> ImageResult<()> {
    target.copy_from(watermark, x, y)?;

    Ok(())
}
