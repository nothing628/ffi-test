use crate::webp_container::{Chunk, RIFFContainer, RegularChunk};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JoinError {
    #[error("Invalid WebP file")]
    InvalidWebpFile,
}

pub fn le_to_u32(inp: &[u8]) -> u32 {
    let len = inp.len();
    let output = match len {
        0 => 0,
        1 => {
            return inp[0].into();
        }
        2 => {
            let mut out: u32 = inp[0].into();
            out = (inp[1] as u32) << 8 | out;

            return out;
        }
        3 => {
            let mut out: u32 = inp[0].into();
            out = (inp[1] as u32) << 8 | out;
            out = (inp[2] as u32) << 16 | out;

            return out;
        }
        4 => {
            let mut out: u32 = inp[0].into();
            out = (inp[1] as u32) << 8 | out;
            out = (inp[2] as u32) << 16 | out;
            out = (inp[3] as u32) << 24 | out;

            return out;
        }
        _ => std::u32::MAX,
    };

    output
}

pub fn be_to_usize(inp: &[u8]) -> usize {
    let len = inp.len();
    let output = match len {
        0 => 0,
        1 => {
            return inp[0].into();
        }
        2 => {
            let mut out: usize = inp[1].into();
            out = (inp[0] as usize) << 8 | out;

            return out;
        }
        3 => {
            let mut out: usize = inp[2].into();
            out = (inp[1] as usize) << 8 | out;
            out = (inp[0] as usize) << 16 | out;

            return out;
        }
        4 => {
            let mut out: usize = inp[3].into();
            out = (inp[2] as usize) << 8 | out;
            out = (inp[1] as usize) << 16 | out;
            out = (inp[0] as usize) << 24 | out;

            return out;
        }
        _ => std::usize::MAX,
    };

    output
}

pub fn usize_to_le(inp: usize) -> [u8; 4] {
    let mut bytes = [0u8; 4];

    bytes[0] = (inp & 0xFF) as u8;
    bytes[1] = ((inp >> 8) & 0xFF) as u8;
    bytes[2] = ((inp >> 16) & 0xFF) as u8;
    bytes[3] = ((inp >> 24) & 0xFF) as u8;

    bytes
}

pub fn usize_to_be(inp: usize) -> [u8; 4] {
    let mut bytes = [0u8; 4];

    bytes[3] = (inp & 0xFF) as u8;
    bytes[2] = ((inp >> 8) & 0xFF) as u8;
    bytes[1] = ((inp >> 16) & 0xFF) as u8;
    bytes[0] = ((inp >> 24) & 0xFF) as u8;

    bytes
}

pub fn join_webp(inp: &[u8], target: &[u8]) -> Result<Vec<u8>, JoinError> {
    let inp_vec = Vec::from(inp);
    let target_vec = Vec::from(target);
    let mut inp_container =
        RIFFContainer::try_from(&inp_vec).map_err(|_| JoinError::InvalidWebpFile)?;

    let regular = RegularChunk {
        chunk_data: target_vec,
        chunk_id: String::from("milf"),
    };

    inp_container.push_subchunk(Box::new(regular));

    Ok(inp_container.to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_to_u32_success() {
        let inp = [0x28u8, 0x7A, 0x44, 0x9A];
        let output = le_to_u32(&inp);

        assert_eq!(output, 0x9A447A28);

        let inp = [0x28u8, 0x7A, 0x44, 0x00];
        let output = le_to_u32(&inp);

        assert_eq!(output, 0x00447A28);

        let inp = [0x00u8, 0x7A, 0x44, 0x9A];
        let output = le_to_u32(&inp);

        assert_eq!(output, 0x9A447A00);
    }

    #[test]
    fn usize_to_le_success() {
        let inp = 0x9A447A28;
        let output = usize_to_le(inp);

        assert_eq!(output, [0x28u8, 0x7A, 0x44, 0x9A]);

        let inp = 0x00447A28;
        let output = usize_to_le(inp);

        assert_eq!(output, [0x28u8, 0x7A, 0x44, 0x00]);

            let inp = 0x9A447A00;
        let output = usize_to_le(inp);

        assert_eq!(output, [0x00u8, 0x7A, 0x44, 0x9A]);
    }
}
