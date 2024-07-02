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

pub fn split_webp(inp_vec: &Vec<u8>) -> Result<(), SplitError> {
    let mut inp_container =
        RIFFContainer::try_from(inp_vec).map_err(|_| SplitError::InvalidWebpFile)?;
    let subchunk = inp_container.find_subchunk("milf");

    if let Some(chunk) = subchunk {
        let chunk_data = chunk.get_chunk_bytes();
        let chunk_len = chunk_data.len();
        let original_img = chunk_data.get(0..chunk_len - 16);
        let watermark_pos = chunk_data.get(chunk_len - 16..chunk_len - 8);
        let watermark_dim = chunk_data.get(chunk_len - 8..chunk_len);

        match (original_img, watermark_pos, watermark_dim) {
            (Some(img_arr), Some(pos_arr), Some(dim_arr)) => {
                let position = Point::try_from(pos_arr)
                    .map_err(|_| -> SplitError {SplitError::CorruptedCustomBlock})?;
                let dimension = Dimension::try_from(dim_arr)
                .map_err(|_| -> SplitError {SplitError::CorruptedCustomBlock})?;

                return Ok(());
            },
            _ => {}
        }

        return Err(SplitError::CorruptedCustomBlock);
    }

    Err(SplitError::CannotFindCustomBlock)
}

pub fn split_jpeg() {
    //
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_split_webp() {
        let content = fs::read("../crop.webp").unwrap();
        let split_result = split_webp(&content);
    }
}
