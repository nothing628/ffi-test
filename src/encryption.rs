use aes::cipher::typenum::{U16, U32};
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;

pub const BASIC_KEY: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
    0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24,
];

fn get_size_to_vec(size: usize) -> [u8; 4] {
    let first_byte = ((size & 0xFF000000) >> 24) as u8;
    let second_byte = ((size & 0xFF0000) >> 16) as u8;
    let third_byte = ((size & 0xFF00) >> 8) as u8;
    let last_byte = (size & 0xFF) as u8;

    [first_byte, second_byte, third_byte, last_byte]
}

fn get_size_from_vec(size: &[u8]) -> usize {
    let mut real_size: usize = 0;
    real_size = real_size | ((size[0] as usize) << 24);
    real_size = real_size | ((size[1] as usize) << 16);
    real_size = real_size | ((size[2] as usize) << 8);
    real_size = real_size | (size[3] as usize);

    real_size
}

fn get_vec_with_size(inp: &[u8]) -> Vec<u8> {
    let inp_size = inp.len();
    let inp_size_vec = get_size_to_vec(inp_size);
    let mut initial_vec = Vec::from(inp_size_vec);

    initial_vec.extend(inp);

    initial_vec
}

fn transform_key(key: &[u8]) -> Result<GenericArray<u8, U32>, String> {
    let mut result: GenericArray<u8, U32> = GenericArray::default();

    for i in 0..32 {
        let item = key.get(i);

        if let Some(val) = item {
            result[i] = *val;
        } else {
            return Err(String::from("Index key not found"));
        }
    }

    Ok(result)
}

fn transform_block(block: &[u8]) -> GenericArray<u8, U16> {
    let mut new_block: GenericArray<u8, U16> = GenericArray::default();

    for i in 0..16 {
        let item = block.get(i).unwrap_or(&0);
        new_block[i] = *item;
    }

    new_block
}

fn get_vec_without_size(inp: &Vec<u8>) -> Option<Vec<u8>> {
    if inp.len() < 5 {
        return None;
    }

    let inp_size = &inp[0..4];
    let real_size = get_size_from_vec(inp_size);

    if real_size > inp.len() - 4 {
        return None;
    }

    let mut result = inp.clone().split_off(4);
    let _ = result.split_off(real_size);

    Some(result)
}

pub fn encrypt(inp: &[u8], key: &[u8]) -> Vec<u8> {
    let copy_inp = get_vec_with_size(&inp);
    let iter = copy_inp.chunks(16);
    let key_block = transform_key(key);
    let mut blocks: Vec<GenericArray<u8, U16>> = Vec::new();
    let mut result: Vec<u8> = Vec::new();

    for val in iter {
        let new_block: GenericArray<u8, U16> = transform_block(val);
        blocks.push(new_block);
    }

    let cipher = Aes256::new(&key_block.unwrap());
    cipher.encrypt_blocks(&mut blocks);

    for block in blocks {
        let block_vec = block.to_vec();
        result.extend(block_vec);
    }

    result
}

pub fn decrypt(inp: &Vec<u8>, key: &Vec<u8>) -> Option<Vec<u8>> {
    let iter = inp.chunks(16);
    let key_block = transform_key(key);
    let mut blocks: Vec<GenericArray<u8, U16>> = Vec::new();
    let mut result: Vec<u8> = Vec::new();

    for val in iter {
        let new_block: GenericArray<u8, U16> = transform_block(val);
        blocks.push(new_block);
    }

    let cipher = Aes256::new(&key_block.unwrap());
    cipher.decrypt_blocks(&mut blocks);

    for block in blocks {
        let block_vec = block.to_vec();
        result.extend(block_vec);
    }

    get_vec_without_size(&result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_size_array() {
        let input_size_vec = get_size_to_vec(0xB2);

        assert_eq!(input_size_vec, [0x00u8, 0x00, 0x00, 0xB2]);

        let input_size_vec = get_size_to_vec(0xFFAA);

        assert_eq!(input_size_vec, [0x00u8, 0x00, 0xFF, 0xAA]);

        let input_size_vec = get_size_to_vec(0xBBFFAA);

        assert_eq!(input_size_vec, [0x00u8, 0xBB, 0xFF, 0xAA]);

        let input_size_vec = get_size_to_vec(0x24CCFFAA);

        assert_eq!(input_size_vec, [0x24u8, 0xCC, 0xFF, 0xAA]);
    }

    #[test]
    fn test_get_size_from_array() {
        let size_arr = [0x00u8, 0x00, 0x00, 0xB2];
        let size = get_size_from_vec(&size_arr);

        assert_eq!(size, 0xB2usize);

        let size_arr = [0x00u8, 0x00, 0xFF, 0xAA];
        let size = get_size_from_vec(&size_arr);

        assert_eq!(size, 0xFFAAusize);

        let size_arr = [0x00u8, 0xBB, 0xFF, 0xAA];
        let size = get_size_from_vec(&size_arr);

        assert_eq!(size, 0xBBFFAAusize);

        let size_arr = [0x24u8, 0xCC, 0xFF, 0xAA];
        let size = get_size_from_vec(&size_arr);

        assert_eq!(size, 0x24CCFFAAusize);
    }

    #[test]
    fn test_add_size_to_vec() {
        let input_vec = vec![0xFF, 0xF2u8];
        let result = get_vec_with_size(&input_vec);

        assert_eq!(result, [0x00u8, 0x00, 0x00, 0x02, 0xFF, 0xF2]);
    }

    #[test]
    fn test_remove_size_from_vec() {
        let input_vec = vec![
            0x00u8, 0x00, 0x00, 0x01, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x1A, 0x1B, 0x1C, 0x1D,
            0x1E, 0x1F,
        ];
        let result = get_vec_without_size(&input_vec).unwrap();
        let expected = [0x0Au8];

        assert_eq!(result, expected);

        let input_arr = [
            0x00u8, 0x00, 0x00, 0x04, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x1A, 0x1B, 0x1C, 0x1D,
            0x1E, 0x1F,
        ];
        let input_vec = Vec::from(input_arr);
        let result = get_vec_without_size(&input_vec).unwrap();
        let expected = [0x0Au8, 0x0B, 0x0C, 0x0D];

        assert_eq!(result, expected);

        let input_arr = [
            0x0Fu8, 0x00, 0x00, 0x04, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x1A, 0x1B, 0x1C, 0x1D,
            0x1E, 0x1F,
        ];
        let input_vec = Vec::from(input_arr);
        let result = get_vec_without_size(&input_vec);

        assert_eq!(result, None);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = vec![
            0xFFu8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
            0xAA, 0xAA, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x22, 0x22, 0x22, 0x22,
            0x22, 0x22, 0x22, 0x22,
        ];

        let data = vec![0xFFu8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let encrypted_data = encrypt(&data, &key);

        assert_eq!(
            encrypted_data,
            [60, 117, 229, 150, 183, 30, 8, 81, 211, 255, 72, 245, 84, 68, 252, 102]
        );

        let decrypted_data = decrypt(&encrypted_data, &key);

        assert_eq!(decrypted_data, Some(data));
    }
}
