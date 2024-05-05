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

#[derive(Debug)]
pub struct RegularChunk {
    pub chunk_id: String,
    pub chunk_data: Vec<u8>,
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

    pub fn push_subchunk(&mut self, chunk: Box<dyn Chunk>) {
        self.subchunks.push(chunk);
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
        let chunk_data = value.get(8..).ok_or_else(|| ChunkError::InvalidChunk)?;
        let result = RegularChunk {
            chunk_id: String::from(chunk_id),
            chunk_data: Vec::from(chunk_data),
        };

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_riff_container() {
        let container = RIFFContainer {
            frame_id: String::from("WEBP"),
            subchunks: Vec::new(),
        };

        assert_eq!(container.frame_id, "WEBP");
        assert_eq!(container.subchunks.len(), 0);
    }

    #[test]
    fn it_can_verify_riff_bytes() {
        let container = RIFFContainer {
            frame_id: String::from("WEBP"),
            subchunks: Vec::new(),
        };
        let container_bytes = container.to_bytes();

        assert_eq!(container_bytes[..], [0x52, 0x49, 0x46, 0x46, 0, 0, 0, 0, 0x57, 0x45, 0x42, 0x50]);
    }

    #[test]
    fn it_can_create_chunk() {
        let chunk = RegularChunk {
            chunk_id: String::from("VP8L"),
            chunk_data: Vec::from([0x52, 0x49, 0x46, 0x46]),
        };
        
        assert_eq!(chunk.chunk_id, "VP8L");
        assert_eq!(chunk.chunk_data.len(), 4);
    }

    #[test]
    fn regular_chunk_return_get_chunk_data()
    {
        let chunk = RegularChunk {
            chunk_id: String::from("VP8L"),
            chunk_data: Vec::from([0x52, 0x49, 0x46, 0x46]),
        };
        let chunk_data = chunk.get_chunk_data();
        
        if let Some(_) = chunk_data {
            panic!("Chunk data for RegularChunk should always be None variant");
        }
    }

    #[test]
    fn regular_chunk_return_get_chunk_bytes() {
        let test_data = [0x52u8, 0x49, 0x46, 0x46];
        let chunk = RegularChunk {
            chunk_id: String::from("VP8L"),
            chunk_data: Vec::from(test_data),
        };
        let chunk_data = chunk.get_chunk_bytes();
        
        assert_eq!(chunk_data, test_data);
    }

    #[test]
    fn regular_chunk_return_get_chunk_id() {
        let test_data = [0x52u8, 0x49, 0x46, 0x46];
        let chunk = RegularChunk {
            chunk_id: String::from("VP8L"),
            chunk_data: Vec::from(test_data),
        };
        let chunk_id = chunk.get_chunk_id();

        assert_eq!(chunk_id, "VP8L");
    }

    #[test]
    fn regular_chunk_return_get_chunk_size() {
        let test_data = [0x52u8, 0x49, 0x46, 0x46];
        let chunk = RegularChunk {
            chunk_id: String::from("VP8L"),
            chunk_data: Vec::from(test_data),
        };
        let chunk_size = chunk.get_chunk_size();

        assert_eq!(chunk_size, 4);
    }

    #[test]
    fn regular_chunk_return_to_bytes() {
        let test_data = [0x52u8, 0x49, 0x46, 0x46];
        let chunk = RegularChunk {
            chunk_id: String::from("VP8L"),
            chunk_data: Vec::from(test_data),
        };
        let chunk_bytes = chunk.to_bytes();

        assert_eq!(chunk_bytes, vec![0x56, 0x50, 0x38, 0x4C, 0x04, 0, 0, 0, 0x52, 0x49, 0x46, 0x46]);
    }


    #[test]
    fn try_from_vec_to_regular_chunk_success() {
        let chunk_bytes = vec![0x56u8, 0x50, 0x38, 0x4C, 0x04, 0, 0, 0, 0x52, 0x49, 0x46, 0x46];
        let chunk = RegularChunk::try_from(&chunk_bytes);

        if let Ok(chunk) = chunk {
            let chunk_data = chunk.get_chunk_data();

            assert_eq!(chunk.get_chunk_id(), "VP8L");
            assert_eq!(chunk.get_chunk_size(), 4);
            assert_eq!(chunk.get_chunk_bytes(), [0x52u8, 0x49, 0x46, 0x46]);

            if let Some(_) = chunk_data {
                panic!("Regular chunk data should be None");
            }
        } else {
            panic!("Convert from vector should be success, but failed");
        }
    }

    #[test]
    fn try_from_vec_to_regular_chunk_failed() {
        let chunk_bytes = vec![0x56u8, 0x50, 0x38];
        let chunk = RegularChunk::try_from(&chunk_bytes);

        if let Ok(_) = chunk {
            panic!("Should failed, chunk_id cannot be identified");
        }

        let chunk_bytes = vec![0x56u8, 0x50, 0x38, 0x4C, 0x04, 0];
        let chunk = RegularChunk::try_from(&chunk_bytes);

        if let Ok(_) = chunk {
            panic!("Should failed, chunk_size cannot be identified");
        }

        let chunk_bytes = vec![0x56u8, 0x50, 0x38, 0x4C, 0x04, 0, 0, 0, 0x52, 0x49];
        let chunk = RegularChunk::try_from(&chunk_bytes);

        if let Ok(_) = chunk {
            panic!("Should failed, chunk_size is lower");
        }

        let chunk_bytes = vec![0x56u8, 0x50, 0x38, 0x4C, 0x04, 0, 0, 0, 0x52, 0x49, 0x46, 0x46, 0x00];
        let chunk = RegularChunk::try_from(&chunk_bytes);

        if let Ok(_) = chunk {
            panic!("Should failed, chunk_size is higher");
        }
    }
}
