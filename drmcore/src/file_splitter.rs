use crate::encryption::decrypt;
use crate::jpeg::container::JFIFContainer;
use crate::jpeg::custom_segment::join_bytes;
use crate::watermark_task::{Dimension, Point};
use crate::webp_container::RIFFContainer;
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

fn process_subchunk(chunk_data: &Vec<u8>, enc_key: &[u8; 32]) -> Result<SplitResult, SplitError> {
    let chunk_decrypted = decrypt(&chunk_data, enc_key).unwrap();
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

    Err(SplitError::CorruptedCustomBlock)
}

pub fn split_webp(inp_vec: &Vec<u8>, enc_key: &[u8;32]) -> Result<SplitResult, SplitError> {
    let mut inp_container =
        RIFFContainer::try_from(inp_vec).map_err(|_| SplitError::InvalidWebpFile)?;
    let subchunk = inp_container.find_subchunk("milf");

    if let Some(chunk) = subchunk {
        let chunk_data = chunk.get_chunk_bytes();

        return process_subchunk(&chunk_data, enc_key);
    }

    Err(SplitError::CannotFindCustomBlock)
}

pub fn split_jpeg(inp_vec: &Vec<u8>, enc_key: &[u8;32]) -> Result<SplitResult, SplitError> {
    let inp_container =
        JFIFContainer::try_from(inp_vec).map_err(|_| SplitError::InvalidJpegFile)?;
    let mut custom_segments = inp_container.get_custom_segment();
    custom_segments.sort_by(|a, b| a.order.cmp(&b.order));
    let subchunk = join_bytes(&custom_segments);
    let subchunk_len = subchunk.len();

    if subchunk_len > 0 {
        return process_subchunk(&subchunk, enc_key);
    }

    Err(SplitError::CannotFindCustomBlock)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encryption::BASIC_KEY;
    use std::fs;

    #[test]
    fn test_split_webp() {
        let content = fs::read("../crop.webp").unwrap();
        let split_result = split_webp(&content, &BASIC_KEY);

        assert_eq!(split_result.is_ok(), true);

        let split_data = split_result.unwrap();

        // fs::write("../somthing.webp", split_data.old_section_img).unwrap();
        println!("position  : {:?}", split_data.position);
        println!("dimension : {:?}", split_data.dimension);
    }

    #[test]
    fn test_split_jpeg() {
        let content = fs::read("../crop.jpeg").unwrap();
        let split_result = split_jpeg(&content, &BASIC_KEY);

        assert_eq!(split_result.is_ok(), true);

        let split_data = split_result.unwrap();

        // fs::write("../somthing.jpeg", split_data.old_section_img).unwrap();
        println!("position  : {:?}", split_data.position);
        println!("dimension : {:?}", split_data.dimension);
    }
}
