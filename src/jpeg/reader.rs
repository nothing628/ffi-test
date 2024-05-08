use crate::file_joiner::be_to_usize;
use crate::jpeg::container::GeneralSegment;
use crate::jpeg::container::JFIFContainer;
use crate::jpeg::container::JFIFSegment;
use crate::jpeg::container::ToBytes;
use anyhow::anyhow;
use anyhow::Result as AnyResult;
use core::iter::Peekable;
use core::slice::Iter;
use thiserror::Error;
use std::convert::{From, TryFrom};

#[derive(Error, Debug)]
pub enum JPEGParserError {
    #[error("File ended prematurely")]
    FileEndedPrematurely,

    #[error("SOI Invalid")]
    InvalidSOI,

    #[error("Invalid segment size")]
    InvalidSegmentSize,

    #[error("Unsupported SOF marker")]
    UnsupportedSOF,

    #[error("Embedded JPGs not supported")]
    UnsupportedEmmbedJPEG,

    #[error("RSTN marker detected before SOS marker")]
    RSTNDetectedBeforeSOS,

    #[error("EOI detected before SOS marker")]
    EOIDetectedBeforeSOS,

    #[error("Unknown marker found : `{0}`")]
    UnknownMarker(u8),

    #[error("Expect marker byte, but not found")]
    ExpectMarker,
}

fn read_segments(iter: &mut Box<&mut Peekable<Iter<u8>>>) -> AnyResult<Vec<u8>> {
    let size_arr = [*iter.next().unwrap(), *iter.next().unwrap()];
    let size = be_to_usize(&size_arr);
    let mut result = Vec::new();

    if size < 2 {
        return Err(anyhow!("Error - Size invalid"));
    }

    for _ in 0..size - 2 {
        result.push(*iter.next().unwrap());
    }

    Ok(result)
}

impl TryFrom<&Vec<u8>> for JFIFContainer {
    type Error = JPEGParserError;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let mut segments: Vec<JFIFSegment> = Vec::new();
        let mut val_iter = value.into_iter().peekable();
        let mut valid = true;
        let mut last_byte = val_iter.next();
        let mut current_byte = val_iter.next();

        if let Some(last_byte_) = last_byte {
            if let Some(current_byte_) = current_byte {
                if JFIFSegment::SOI.get_marker().unwrap() == [*last_byte_, *current_byte_] {
                    segments.push(JFIFSegment::SOI);
                } else {
                    return Err(JPEGParserError::InvalidSOI);
                }
            } else {
                valid = false;
            }
        } else {
            valid = false;
        }

        last_byte = val_iter.next();
        current_byte = val_iter.next();

        while valid {
            match val_iter.peek() {
                None => {
                    return Err(JPEGParserError::FileEndedPrematurely);
                }
                _ => {}
            }

            let last_byte_ = last_byte.unwrap();
            let mut current_byte_ = current_byte.unwrap();

            if *last_byte_ != 0xFF {
                return Err(JPEGParserError::ExpectMarker);
            }

            while *last_byte_ == 0xFF && *current_byte_ == 0xFF {
                current_byte = val_iter.next();
                current_byte_ = current_byte.unwrap();
            }

            match current_byte_ {
                0xC0 => {
                    let mut iter = Box::new(&mut val_iter);
                    match read_segments(&mut iter) {
                        Ok(data) => {
                            segments.push(JFIFSegment::SOF0(GeneralSegment::new(data)));
                        }
                        Err(_) => {
                            return Err(JPEGParserError::InvalidSegmentSize);
                        }
                    }
                }
                0xC2 => {
                    let mut iter = Box::new(&mut val_iter);
                    match read_segments(&mut iter) {
                        Ok(data) => {
                            segments.push(JFIFSegment::SOF2(GeneralSegment::new(data)));
                        }
                        Err(_) => {
                            return Err(JPEGParserError::InvalidSegmentSize);
                        }
                    }
                }
                0xC4 => {
                    let mut iter = Box::new(&mut val_iter);
                    match read_segments(&mut iter) {
                        Ok(data) => {
                            segments.push(JFIFSegment::DHT(GeneralSegment::new(data)));
                        }
                        Err(_) => {
                            return Err(JPEGParserError::InvalidSegmentSize);
                        }
                    }
                }
                0xC1 | 0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF => {
                    return Err(JPEGParserError::UnsupportedSOF);
                }
                0xD0..=0xD7 => {
                    return Err(JPEGParserError::RSTNDetectedBeforeSOS);
                }
                0xD8 => {
                    return Err(JPEGParserError::UnsupportedEmmbedJPEG);
                }
                0xD9 => {
                    return Err(JPEGParserError::EOIDetectedBeforeSOS);
                }
                0xDA => {
                    // SOS
                    let mut iter = Box::new(&mut val_iter);
                    match read_segments(&mut iter) {
                        Ok(data) => {
                            segments.push(JFIFSegment::SOS(GeneralSegment::new(data)));
                        }
                        Err(_) => {
                            return Err(JPEGParserError::InvalidSegmentSize);
                        }
                    }
                    break;
                }
                0xDB => {
                    let mut iter = Box::new(&mut val_iter);
                    match read_segments(&mut iter) {
                        Ok(data) => {
                            segments.push(JFIFSegment::DQT(GeneralSegment::new(data)));
                        }
                        Err(_) => {
                            return Err(JPEGParserError::InvalidSegmentSize);
                        }
                    }
                }
                0xDD => {
                    val_iter.next();
                    val_iter.next();
                    let first = val_iter.next().unwrap();
                    let second = val_iter.next().unwrap();
                    segments.push(JFIFSegment::DRI([*first, *second]));
                }
                0xE0..=0xEF => {
                    let app_num = current_byte_ & 0x0F;
                    let mut iter = Box::new(&mut val_iter);
                    match read_segments(&mut iter) {
                        Ok(data) => {
                            segments.push(JFIFSegment::APP(app_num, GeneralSegment::new(data)));
                        }
                        Err(_) => {
                            return Err(JPEGParserError::InvalidSegmentSize);
                        }
                    }
                }
                0xFE => {
                    let mut iter = Box::new(&mut val_iter);
                    match read_segments(&mut iter) {
                        Ok(data) => {
                            segments.push(JFIFSegment::COM(GeneralSegment::new(data)));
                        }
                        Err(_) => {
                            return Err(JPEGParserError::InvalidSegmentSize);
                        }
                    }
                }
                _ => {
                    return Err(JPEGParserError::UnknownMarker(*current_byte_));
                }
            }

            last_byte = val_iter.next();
            current_byte = val_iter.next();
        }

        let mut last_byte_ = *last_byte.unwrap();
        let mut current_byte_ = *current_byte.unwrap();
        let mut img_data: Vec<u8> = Vec::new();

        // SOS already found, start read until found EOI
        loop {
            if last_byte_ == 0xFF && current_byte_ == 0xD9 {
                segments.push(JFIFSegment::IMGDATA(img_data));
                segments.push(JFIFSegment::EOI);
                break;
            } else {
                img_data.push(last_byte_);
                last_byte_ = current_byte_;
                current_byte = val_iter.next();
                current_byte_ = *current_byte.unwrap();
            }
        }

        let result = JFIFContainer::new(segments);

        Ok(result)
    }
}

// impl From<&Vec<u8>> for JFIFContainer {
//     fn from(value: &Vec<u8>) -> Self {
//         let segments = Vec::new();

//         JFIFContainer::new(segments)
//     }
// }