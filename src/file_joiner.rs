use thiserror::Error;

#[derive(Error, Debug)]
pub enum JoinError {
    #[error("Invalid WebP file")]
    InvalidWebpFile,
}

fn le_to_u32(inp: &[u8]) -> u32 {
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

fn usize_to_le(inp: usize) -> [u8; 4] {
    let mut bytes = [0u8; 4];

    bytes[0] = (inp & 0xFF) as u8;
    bytes[1] = ((inp >> 8) & 0xFF) as u8;
    bytes[2] = ((inp >> 16) & 0xFF) as u8;
    bytes[3] = ((inp >> 24) & 0xFF) as u8;

    bytes
}

fn create_subchunk(chunk_id: &str, data: &[u8]) -> Vec<u8> {
    let chunk_size = data.len();
    let chunk_size_bytes = usize_to_le(chunk_size);
    let chunk_id_bytes = chunk_id.as_bytes();
    let mut result = Vec::from(chunk_id_bytes);

    chunk_size_bytes.iter().for_each(|f| result.push(*f));
    data.iter().for_each(|f| result.push(*f));

    result
}

pub fn join_webp(inp: &[u8], target: &[u8]) -> Result<Vec<u8>, JoinError> {
    let result = Vec::new();
    let size_slice = inp.get(4..8).ok_or_else(|| JoinError::InvalidWebpFile)?;
    let chunk = create_subchunk("milf", &target[..]);
    let size = le_to_u32(&size_slice);

    Ok(result)
}
