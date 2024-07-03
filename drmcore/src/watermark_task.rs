use crate::file_joiner::{le_to_u32, usize_to_le};
use anyhow::{anyhow, Result};
use image::{
    load_from_memory_with_format, DynamicImage, GenericImage, GenericImageView, ImageFormat,
    ImageResult, Pixel,
};
use thiserror::Error;

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

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Invalid type")]
    ConversionError,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn solve_absolute_position(
    target: &Dimension,
    watermark: &Dimension,
    origin_x: &OriginX,
    origin_y: &OriginY,
    offset: &Point,
) -> Point {
    let offset_x = offset.x;
    let offset_y = offset.y;
    let abs_x = if *origin_x == OriginX::Left {
        offset_x
    } else {
        target.width - watermark.width - offset_x
    };
    let abs_y = if *origin_y == OriginY::Top {
        offset_y
    } else {
        target.height - watermark.height - offset_y
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

    pub fn get_watermark_dimension(&self) -> Option<Dimension> {
        match &self.watermark {
            Some(watermark_img) => {
                let (watermark_w, watermark_h) = watermark_img.dimensions();

                Some(Dimension::new(watermark_w, watermark_h))
            }
            _ => None,
        }
    }

    pub fn get_target_dimension(&self) -> Option<Dimension> {
        match &self.target {
            Some(target_img) => {
                let (width, height) = target_img.dimensions();

                Some(Dimension::new(width, height))
            }
            _ => None,
        }
    }

    pub fn get_absolute_watermark_position(&self) -> Option<Point> {
        let offset_x = self.x;
        let offset_y = self.y;
        let offset = Point {
            x: offset_x,
            y: offset_y,
        };
        let origin_x = &self.origin_x;
        let origin_y = &self.origin_y;
        let watermark_dim = self.get_watermark_dimension();
        let target_dim = self.get_target_dimension();

        match (target_dim, watermark_dim) {
            (Some(target_dim), Some(watermark_dim)) => {
                let pos = solve_absolute_position(
                    &target_dim,
                    &watermark_dim,
                    origin_x,
                    origin_y,
                    &offset,
                );

                return Some(pos);
            }
            _ => {}
        }

        None
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
        let target = &self.target;
        let watermark = &self.watermark;

        match (target, watermark) {
            (Some(target_img), Some(watermark_img)) => {
                let (watermark_w, watermark_h) = watermark_img.dimensions();
                let position = self.get_absolute_watermark_position().unwrap();
                let mut clone_target = target_img.clone();
                let mut sub_img =
                    clone_target.sub_image(position.x, position.y, watermark_w, watermark_h);
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
            }
            _ => Err(anyhow!("Watermark or target image is not set")),
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

impl From<Point> for [u8; 8] {
    fn from(value: Point) -> Self {
        let x = usize_to_le(value.x as usize);
        let y = usize_to_le(value.y as usize);

        [x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]]
    }
}

impl From<Dimension> for [u8; 8] {
    fn from(value: Dimension) -> Self {
        let w = usize_to_le(value.width as usize);
        let h = usize_to_le(value.height as usize);

        [w[0], w[1], w[2], w[3], h[0], h[1], h[2], h[3]]
    }
}

impl TryFrom<&[u8]> for Dimension {
    type Error = ConversionError;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let w = value.get(0..4);
        let h = value.get(4..8);

        return match (w, h) {
            (Some(w), Some(h)) => {
                let width = le_to_u32(w);
                let height = le_to_u32(h);

                return Ok(Dimension { height, width });
            }
            _ => Err(ConversionError::ConversionError),
        };
    }
}

impl TryFrom<&[u8]> for Point {
    type Error = ConversionError;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let x = value.get(0..4);
        let y = value.get(4..8);

        return match (x, y) {
            (Some(x), Some(y)) => {
                let x_pos = le_to_u32(x);
                let y_pos = le_to_u32(y);

                return Ok(Point { x: x_pos, y: y_pos });
            }
            _ => Err(ConversionError::ConversionError),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jpeg_watermark_task() {
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

        let watermark_pos: [u8; 8] = watermark_task
            .get_absolute_watermark_position()
            .unwrap()
            .into();
        let watermark_dim: [u8; 8] = watermark_task.get_watermark_dimension().unwrap().into();

        println!("position {:?}", watermark_pos);
        println!("dimension {:?}", watermark_dim);

        // // The dimensions method returns the images width and height.
        // println!("dimensions {:?}", img.dimensions());

        // // The color method returns the image's `ColorType`.
        // println!("{:?}", img.color());

        // // Write the contents of this image to the Writer in PNG format.
        // img.save("../testx.jpeg").unwrap();
    }
}
