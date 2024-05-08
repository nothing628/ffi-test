use std::convert::From;
use super::container::{GeneralSegment, JFIFSegment};

pub const CUSTOM_SEGMENT_APP: u8 = 10;
pub const CUSTOM_SEGMENT_NAME: &str = "MILF";
pub const CUSTOM_SEGMENT_MAX_SIZE: u16 = 0xFFFF - 0x23; // 65500 bytes

pub struct CustomSegment {
    data: Vec<u8>,
    order: u16,
}

impl From<CustomSegment> for JFIFSegment {
    fn from(value: CustomSegment) -> Self {
        let mut data = Vec::from(CUSTOM_SEGMENT_NAME.as_bytes());
        let order = value.order.to_be_bytes();
        
        data.push(0x00);
        data.extend(order);
        data.extend(value.data);

        let segment = GeneralSegment::new(data);

        JFIFSegment::APP(CUSTOM_SEGMENT_APP, segment)
    }
}

#[cfg(test)]
mod tests {
    use crate::jpeg::container::ToBytes as _;
    use super::*;

    #[test]
    fn custom_segment_to_app_segment() {
        let segment = CustomSegment {
            data: vec![0xFF, 0xBA, 0x28],
            order: 0,
        };
        let app_segment = JFIFSegment::from(segment);

        match app_segment {
            JFIFSegment::APP(app,data ) => {
                let data_vec = data.to_bytes();

                assert_eq!(app, CUSTOM_SEGMENT_APP);
                assert_eq!(data_vec, [0x00, 0x0C, 0x4D, 0x49, 0x4C, 0x46, 0x00, 0x00, 0x00, 0xFF, 0xBA, 0x28]);
            },
            _ => panic!("JFIF Segment not APP")
        }
    }
}