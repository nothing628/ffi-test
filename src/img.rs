use std::io::Cursor;
use image::load_from_memory;

pub fn get_section_webp(img: &[u8], x: u32, y: u32, w: u32, h:u32) -> Vec<u8> {
    let img = load_from_memory(img).unwrap();
    let crop_img = img.crop_imm(x, y, w, h);
    let mut bytes: Vec<u8> = Vec::new();
    let mut cur = Cursor::new(&mut bytes);
    crop_img.write_to(&mut cur, image::ImageFormat::WebP).unwrap();

    bytes
}
