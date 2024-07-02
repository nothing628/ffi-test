use crate::webp_container::{Chunk, RIFFContainer, RegularChunk};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SplitError {
    #[error("Invalid WebP file")]
    InvalidWebpFile,

    #[error("Invalid Jpeg container")]
    InvalidJpegFile,

    #[error("Cannot bind to jpeg file")]
    CannotInsertCustomSegment,
}

pub fn split_webp(inp_vec: &Vec<u8>) -> Result<(), SplitError> {
    let mut inp_container =
        RIFFContainer::try_from(inp_vec).map_err(|_| SplitError::InvalidWebpFile)?;

    Ok(())
}

pub fn split_jpeg() {
    //
}
