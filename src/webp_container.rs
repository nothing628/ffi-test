use std::convert::TryFrom;
use std::str;
use thiserror::Error;

use crate::file_joiner::{le_to_u32, usize_to_le};

#[derive(Debug, Error)]
pub enum RiffContainerError {
    #[error("Invalid riff file")]
    InvalidRiffFile,

    #[error("Size doesn't match")]
    SizeMismatch,

    #[error("Missing header")]
    MissingHeader,
}

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("Invalid chunk")]
    InvalidChunk,

    #[error("Size doesn't match")]
    SizeMismatch,
}

pub struct RIFFContainer {
    frame_id: String,
    subchunks: Vec<Box<dyn Chunk>>,
}

pub struct RegularChunk {
    chunk_id: String,
    chunk_data: Vec<u8>,
}

pub trait Chunk {
    fn get_chunk_id(&self) -> &str;
    fn get_chunk_size(&self) -> usize;
    fn get_chunk_data(&self) -> Option<&Vec<Box<dyn Chunk>>>;
    fn get_chunk_bytes(&self) -> Vec<u8>;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait RiffChunk {
    fn get_chunk_frame_id(&self) -> &str;
}

impl RIFFContainer {
    pub fn generate_sub_chunk(inp: &Vec<u8>) -> Vec<Box<dyn Chunk>> {
        let mut result = Vec::new();
        let mut counter = 0usize;
        let inp_size = inp.len();

        while counter < inp_size {
            let size_start = counter + 4;
            let size_end = counter + 8;
            let chunk_size_opt = inp.get(size_start..size_end);
            
            if let Some(chunk_size) = chunk_size_opt {
                let real_chunk_size = le_to_u32(chunk_size) as usize;
                let chunk_start = counter;
                let chunk_end = counter + 8 + real_chunk_size;
                let chunk_slice_opt = inp.get(chunk_start..chunk_end);
                
                if let Some(chunk_slice) = chunk_slice_opt {
                    let chunk_vec = Vec::from(chunk_slice);
                    let chunk = RegularChunk::try_from(&chunk_vec);

                    if let Ok(chunk) = chunk {
                        let chunk_box: Box<dyn Chunk> = Box::new(chunk);
                        result.push(chunk_box);

                        counter = chunk_end;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        result
    }
}

impl RiffChunk for RIFFContainer {
    fn get_chunk_frame_id(&self) -> &str {
        self.frame_id.as_str()
    }
}

impl Chunk for RIFFContainer {
    fn get_chunk_id(&self) -> &str {
        "RIFF"
    }

    fn get_chunk_data(&self) -> Option<&Vec<Box<dyn Chunk>>> {
        Some(&self.subchunks)
    }

    fn get_chunk_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let subchunk = &self.subchunks;

        subchunk.iter().for_each(|f| {
            let bytes = f.get_chunk_bytes();
            bytes.iter().for_each(|g| result.push(*g));
        });

        result
    }

    fn get_chunk_size(&self) -> usize {
        let chunk_bytes = self.get_chunk_bytes();
        chunk_bytes.len()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let chunk_id_bytes = self.get_chunk_id().as_bytes();
        let chunk_size = usize_to_le(self.get_chunk_size());
        let chunk_bytes = self.get_chunk_bytes();
        let frame_id = self.frame_id.as_bytes();

        chunk_id_bytes.iter().for_each(|f| result.push(*f));
        chunk_size.iter().for_each(|f| result.push(*f));
        frame_id.iter().for_each(|f| result.push(*f));
        chunk_bytes.iter().for_each(|f| result.push(*f));

        result
    }
}

impl Chunk for RegularChunk {
    fn get_chunk_data(&self) -> Option<&Vec<Box<dyn Chunk>>> {
        None
    }

    fn get_chunk_bytes(&self) -> Vec<u8> {
        self.chunk_data.clone()
    }

    fn get_chunk_id(&self) -> &str {
        &self.chunk_id
    }

    fn get_chunk_size(&self) -> usize {
        self.chunk_data.len()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let chunk_id_bytes = self.get_chunk_id().as_bytes();
        let chunk_size = usize_to_le(self.get_chunk_size());
        let chunk_bytes = &self.chunk_data;

        chunk_id_bytes.iter().for_each(|f| result.push(*f));
        chunk_size.iter().for_each(|f| result.push(*f));
        chunk_bytes.iter().for_each(|f| result.push(*f));

        result
    }
}

impl TryFrom<&Vec<u8>> for RIFFContainer {
    type Error = RiffContainerError;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let first_header = value
            .get(0..4)
            .ok_or_else(|| RiffContainerError::InvalidRiffFile)?;

        if first_header != "RIFF".as_bytes() {
            return Err(RiffContainerError::MissingHeader);
        }

        let size_id = value
            .get(4..8)
            .ok_or_else(|| RiffContainerError::InvalidRiffFile)?;
        let chunk_size = le_to_u32(size_id) as usize;
        let vec_size = value.len();

        if chunk_size + 8 != vec_size {
            return Err(RiffContainerError::SizeMismatch);
        }

        let frame_id_bytes = value
            .get(8..12)
            .ok_or_else(|| RiffContainerError::InvalidRiffFile)?;
        let frame_id = str::from_utf8(frame_id_bytes).unwrap();
        let subchunk_slice = value.get(12..).ok_or_else(|| RiffContainerError::InvalidRiffFile)?;
        let subchunk = RIFFContainer::generate_sub_chunk(&Vec::from(subchunk_slice));

        Ok(RIFFContainer {
            frame_id: String::from(frame_id),
            subchunks: subchunk,
        })
    }
}

impl TryFrom<&Vec<u8>> for RegularChunk {
    type Error = ChunkError;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let first_header = value
            .get(0..4)
            .ok_or_else(|| ChunkError::InvalidChunk)?;
        let size_id = value
            .get(4..8)
            .ok_or_else(|| ChunkError::InvalidChunk)?;
        let chunk_size = le_to_u32(size_id) as usize;
        let vec_size = value.len();

        if chunk_size + 8 != vec_size {
            return Err(ChunkError::SizeMismatch);
        }

        let chunk_id = str::from_utf8(first_header).unwrap();
        let chunk_data = value.get(8..).ok_or_else(|| ChunkError::InvalidChunk)?;;
        let result = RegularChunk {
            chunk_id: String::from(chunk_id),
            chunk_data: Vec::from(chunk_data),
        };

        Ok(result)
    }
}