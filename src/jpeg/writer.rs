use super::container::JFIFContainer;
use super::container::ToBytes;

impl From<JFIFContainer> for Vec<u8> {
    fn from(value: JFIFContainer) -> Self {
        let mut result = Vec::new();

        value.get_segments().iter().for_each(|f| {
            let bytes = f.to_bytes();
            result.extend_from_slice(&bytes[..]);
        });

        result
    }
}
