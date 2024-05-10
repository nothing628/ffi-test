use std::convert::{From, TryFrom};
use thiserror::Error;
use super::container::{GeneralSegment, JFIFSegment};
use crate::file_joiner::be_to_usize;

pub const CUSTOM_SEGMENT_APP: u8 = 10;
pub const CUSTOM_SEGMENT_NAME: &str = "MILF";
pub const CUSTOM_SEGMENT_MAX_SIZE: u16 = 0xFFFF - 0x23; // 65500 bytes

#[derive(Debug, PartialEq)]
pub struct CustomSegment {
    data: Vec<u8>,
    order: u16,
}

#[derive(Error, Debug, PartialEq)]
pub enum CustomSegmentError {
    #[error("Invalid Segment type")]
    InvalidSegmentType,

    #[error("Invalid App Num")]
    InvalidAppNum,

    #[error("Segment missing order and data")]
    EmptyDataOrOrder,
}

impl CustomSegment {
    pub fn new(data: &[u8], order: u16) -> Self {
        Self {
            data: Vec::from(data),
            order,
        }
    }
}

pub fn split_bytes(data: &[u8]) -> Vec<CustomSegment> {
    let mut result = Vec::new();
    let mut order = 0u16;

    data.chunks(CUSTOM_SEGMENT_MAX_SIZE.into())
        .for_each(|f| {
            result.push(CustomSegment::new(f, order));
            order = order + 1;
        });

    result
}

pub fn join_bytes(data: &Vec<CustomSegment>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut vec: Vec<&CustomSegment> = Vec::new();

    data.iter().for_each(|f| {vec.push(f)});
    vec.sort_by(|a, b| a.order.cmp(&b.order));
    vec.iter()
        .for_each(|f| {
            result.extend(&f.data);
        });

    result
}

impl From<&CustomSegment> for JFIFSegment {
    fn from(value: &CustomSegment) -> Self {
        let mut data = Vec::from(CUSTOM_SEGMENT_NAME.as_bytes());
        let order = value.order.to_be_bytes();
        
        data.push(0x00);
        data.extend(order);
        data.extend(&value.data);

        let segment = GeneralSegment::new(data);

        JFIFSegment::APP(CUSTOM_SEGMENT_APP, segment)
    }
}

impl TryFrom<&JFIFSegment> for CustomSegment {
    type Error = CustomSegmentError;

    fn try_from(value: &JFIFSegment) -> Result<Self, Self::Error> {
        match value {
            JFIFSegment::APP(app_num, data) => {
                if *app_num != CUSTOM_SEGMENT_APP {
                    return Err(CustomSegmentError::InvalidAppNum);
                }

                let raw_data = data.get_data();
                let raw_order = raw_data.get(5..7);
                let raw_bytes = raw_data.get(7..);

                match (raw_order,raw_bytes) {
                    (Some(order), Some(bytes)) => {
                        let order_be = be_to_usize(order) as u16;
                        let custom_segment = CustomSegment {
                            order: order_be,
                            data: Vec::from(bytes),
                        };
        
                        return Ok(custom_segment);
                    },
                    _ => {
                        return Err(CustomSegmentError::EmptyDataOrOrder);
                    }
                }
            },
            _ => {
                return Err(CustomSegmentError::InvalidSegmentType);
            }
        }
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
        let app_segment = JFIFSegment::from(&segment);

        match app_segment {
            JFIFSegment::APP(app,data ) => {
                let data_vec = data.to_bytes();

                assert_eq!(app, CUSTOM_SEGMENT_APP);
                assert_eq!(data_vec, [0x00, 0x0C, 0x4D, 0x49, 0x4C, 0x46, 0x00, 0x00, 0x00, 0xFF, 0xBA, 0x28]);
            },
            _ => panic!("JFIF Segment not APP")
        }
    }

    #[test]
    fn jfif_segment_to_custom_segment_should_error() {
        let jfif_segment = JFIFSegment::SOI;
        let try_custom_segment = CustomSegment::try_from(&jfif_segment);

        match try_custom_segment {
            Ok(_) => {
                panic!("Conversion using other segment type except APP segment should fail");
            }
            Err(err) => {
                assert_eq!(err, CustomSegmentError::InvalidSegmentType);
            }
        }

        let data = GeneralSegment::new(vec![0xFF, 0xFF, 0x12]);
        let app_segment = JFIFSegment::APP(0, data);
        let try_custom_segment = CustomSegment::try_from(&app_segment);

        match try_custom_segment {
            Ok(_) => {
                panic!("Conversion using other app num should fail");
            }
            Err(err) => {
                assert_eq!(err, CustomSegmentError::InvalidAppNum);
            }
        }

        let data = GeneralSegment::new(vec![0xFF, 0xFF, 0x12]);
        let app_segment = JFIFSegment::APP(CUSTOM_SEGMENT_APP, data);
        let try_custom_segment = CustomSegment::try_from(&app_segment);

        match try_custom_segment {
            Ok(_) => {
                panic!("Should fail because can find data or size");
            }
            Err(err) => {
                assert_eq!(err, CustomSegmentError::EmptyDataOrOrder);
            }
        }
    }

    #[test]
    fn jfif_segment_to_custom_segment_should_success() {
        let header = CUSTOM_SEGMENT_NAME.as_bytes();
        let mut data = Vec::from(header);
        data.push(0x00);
        data.push(0x00);
        data.push(0x10);
        data.push(0x00);
        let general_segment = GeneralSegment::new(data);
        let jfif_segment = JFIFSegment::APP(CUSTOM_SEGMENT_APP, general_segment);
        let try_custom_segment = CustomSegment::try_from(&jfif_segment);

        match try_custom_segment {
            Ok(custom_segment) => {
                assert_eq!(custom_segment.order, 16);
                assert_eq!(custom_segment.data, [0x00]);
            },
            Err(err) => {
                panic!("Should not error converting to custom_segment : {}", err);
            }
        }
    }
}