use crate::encryption::{decrypt, BASIC_KEY};
use crate::jpeg::container::JFIFContainer;
use crate::watermark_task::{Dimension, Point};
use crate::webp_container::{Chunk, RIFFContainer, RegularChunk};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SplitError {
    #[error("Invalid WebP file")]
    InvalidWebpFile,

    #[error("Invalid Jpeg container")]
    InvalidJpegFile,

    #[error("Cannot find custom block")]
    CannotFindCustomBlock,

    #[error("Corrupted custom block")]
    CorruptedCustomBlock,
}

pub struct SplitResult {
    pub position: Point,
    pub dimension: Dimension,
    pub old_section_img: Vec<u8>,
}

pub fn split_webp(inp_vec: &Vec<u8>) -> Result<SplitResult, SplitError> {
    let mut inp_container =
        RIFFContainer::try_from(inp_vec).map_err(|_| SplitError::InvalidWebpFile)?;
    let subchunk = inp_container.find_subchunk("milf");

    if let Some(chunk) = subchunk {
        let chunk_data = chunk.get_chunk_bytes();
        let chunk_decrypted = decrypt(&chunk_data, &BASIC_KEY).unwrap();
        let chunk_len = chunk_decrypted.len();
        let original_img = chunk_decrypted.get(0..chunk_len - 16);
        let watermark_pos = chunk_decrypted.get(chunk_len - 16..chunk_len - 8);
        let watermark_dim = chunk_decrypted.get(chunk_len - 8..chunk_len);

        match (original_img, watermark_pos, watermark_dim) {
            (Some(img_arr), Some(pos_arr), Some(dim_arr)) => {
                let position = Point::try_from(pos_arr)
                    .map_err(|_| -> SplitError { SplitError::CorruptedCustomBlock })?;
                let dimension = Dimension::try_from(dim_arr)
                    .map_err(|_| -> SplitError { SplitError::CorruptedCustomBlock })?;

                let split_result = SplitResult {
                    dimension,
                    position,
                    old_section_img: Vec::from(img_arr),
                };

                return Ok(split_result);
            }
            _ => {}
        }

        return Err(SplitError::CorruptedCustomBlock);
    }

    Err(SplitError::CannotFindCustomBlock)
}

pub fn split_jpeg(inp_vec: &Vec<u8>) -> Result<SplitResult, SplitError> {
    let inp_container = JFIFContainer::try_from(inp_vec).map_err(|_| SplitError::InvalidJpegFile)?;
    let custom_segments = inp_container.get_custom_segment();
    
    Err(SplitError::CannotFindCustomBlock)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_split_webp() {
        let content = fs::read("../crop.webp").unwrap();
        let split_result = split_webp(&content);

        match split_result {
            Ok(split_data) => {
                fs::write("../somthing.webp", split_data.old_section_img).unwrap();
                println!("position  : {:?}", split_data.position);
                println!("dimension : {:?}", split_data.dimension);
            }
            Err(err) => {
                //
            }
        }
    }

    #[test]
    fn test_split_jpeg() {
        let content = fs::read("../crop.jpeg").unwrap();
        let split_result = split_jpeg(&content);

        match split_result {
            Ok(split_data) => {
                fs::write("../somthing.jpeg", split_data.old_section_img).unwrap();
                println!("position  : {:?}", split_data.position);
                println!("dimension : {:?}", split_data.dimension);
            }
            Err(err) => {
                //
            }
        }
    }
}
