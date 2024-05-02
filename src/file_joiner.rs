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
        },
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

pub fn join_webp(inp: &Vec<u8>, target: &Vec<u8>) -> Result<Vec<u8>, JoinError> {
    let result = Vec::new();
    let size_slice = inp.get(4..8).ok_or_else(|| JoinError::InvalidWebpFile)?;
    let size = le_to_u32(&size_slice);

    Ok(result)
}
