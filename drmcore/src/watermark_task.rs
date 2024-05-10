use anyhow::{anyhow, Result};
use image::{
    load_from_memory_with_format, DynamicImage, GenericImage, GenericImageView, ImageFormat,
    ImageResult, Pixel,
};

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
    old_section: Option<DynamicImage>,
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
            old_section: None,
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

    pub fn get_old_section(&self) -> &Option<DynamicImage> {
        &self.old_section
    }

    pub fn get_output(&self) -> &Option<DynamicImage> {
        &self.output
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
                let (watermark_w, watermark_h) = watermark_img.dimensions();
                let pos = solve_absolute_position(&target_img, &watermark_img, origin_x, origin_y, &offset);
                let mut clone_target = target_img.clone();
                let mut sub_img = clone_target.sub_image(pos.x, pos.y, watermark_w, watermark_h);
                let copy_sub_img: DynamicImage = sub_img.to_image().into();

                for x in 0..watermark_w {
                    for y in 0..watermark_h {
                        let pix_src = watermark_img.get_pixel(x, y);
                        let mut pix_tar = sub_img.get_pixel(x, y);
                        pix_tar.blend(&pix_src);
                        sub_img.put_pixel(x, y, pix_tar);
                    }
                }

                self.output = Some(clone_target);
                self.old_section = Some(copy_sub_img);

                Ok(())
            } else {
                Err(anyhow!("Watermark is not set"))
            }
        } else {
            Err(anyhow!("Target is not set"))
        }
    }
}

pub fn set_watermark(
    watermark_task: &mut WatermarkTask,
    bytes: &[u8],
    format: ImageFormat,
) -> ImageResult<()> {
    let watermark = load_from_memory_with_format(bytes, format)?;
    watermark_task.set_watermark(Some(watermark));

    Ok(())
}

pub fn set_target(
    watermark_task: &mut WatermarkTask,
    bytes: &[u8],
    format: ImageFormat,
) -> ImageResult<()> {
    let target = load_from_memory_with_format(bytes, format)?;
    watermark_task.set_target(Some(target));

    Ok(())
}
