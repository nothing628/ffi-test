use crate::file_joiner::be_to_usize;
use crate::file_joiner::usize_to_be;
use anyhow::anyhow;
use anyhow::Result;
use core::convert::From;
use core::slice::Iter;
use std::iter::Peekable;

pub struct GeneralSegment {
    data: Vec<u8>,
}

pub enum JFIFSegment {
    SOI,
    SOF0(GeneralSegment),
    SOF2(GeneralSegment),
    DHT(GeneralSegment),
    DQT(GeneralSegment),
    DRI([u8; 2]),
    SOS(GeneralSegment),
    RST(u8),
    APP(u8, GeneralSegment),
    COM(GeneralSegment),
    IMGDATA(Vec<u8>),
    EOI,
}

pub struct JFIFContainer {
    segments: Vec<JFIFSegment>,
}

impl From<JFIFContainer> for Vec<u8> {
    fn from(value: JFIFContainer) -> Self {
        let mut result = Vec::new();

        value.segments.iter().for_each(|f| {
            let bytes = f.to_bytes();
            result.extend_from_slice(&bytes[..]);
        });

        result
    }
}

fn read_segments(iter: &Box<&Peekable<Iter<u8>>>) -> Result<Vec<u8>> {
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

impl From<&Vec<u8>> for JFIFContainer {
    fn from(value: &Vec<u8>) -> Self {
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
                    // Error - SOI invalid
                    valid = false;
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
                    // Error - File ended prematurely
                    valid = false;
                    break;
                }
                _ => {}
            }

            let last_byte_ = last_byte.unwrap();
            let mut current_byte_ = current_byte.unwrap();
            let marker = [last_byte_, current_byte_];

            if *last_byte_ != 0xFF {
                valid = false;
                break;
            }

            match last_byte_ {
                0xFF => {
                    match current_byte_ {
                        0xC0 => {
                            let iter: Box<&Peekable<Iter<u8>>> = Box::new(&val_iter);
                            match read_segments(&iter) {
                                Ok(data) => {
                                    segments.push(JFIFSegment::SOF0(GeneralSegment { data }));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        0xC2 => {
                            let iter: Box<&Peekable<Iter<u8>>> = Box::new(&val_iter);
                            match read_segments(&iter) {
                                Ok(data) => {
                                    segments.push(JFIFSegment::SOF2(GeneralSegment { data }));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        0xC4 => {
                            let iter: Box<&Peekable<Iter<u8>>> = Box::new(&val_iter);
                            match read_segments(&iter) {
                                Ok(data) => {
                                    segments.push(JFIFSegment::DHT(GeneralSegment { data }));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        0xC1 | 0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF => {
                            // Error - SOF marker not supported
                            valid = false;
                            break;
                        }
                        0xD0..=0xD7 => {
                            // Error - RSTN detected before SOS
                            valid = false;
                            break;
                        }
                        0xD8 => {
                            // Error - Embedded JPGs not supported
                            valid = false;
                            break;
                        }
                        0xD9 => {
                            // Error - EOI detected before SOS
                            valid = false;
                            break;
                        }
                        0xDA => {
                            // SOS
                            break;
                        }
                        0xDB => {
                            let iter: Box<&Peekable<Iter<u8>>> = Box::new(&val_iter);
                            match read_segments(&iter) {
                                Ok(data) => {
                                    segments.push(JFIFSegment::DQT(GeneralSegment { data }));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
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
                            let iter: Box<&Peekable<Iter<u8>>> = Box::new(&val_iter);
                            match read_segments(&iter) {
                                Ok(data) => {
                                    segments
                                        .push(JFIFSegment::APP(app_num, GeneralSegment { data }));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        0xFE => {
                            let iter: Box<&Peekable<Iter<u8>>> = Box::new(&val_iter);
                            match read_segments(&iter) {
                                Ok(data) => {
                                    segments.push(JFIFSegment::COM(GeneralSegment { data }));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
                                }
                            }
                        }
                        0xFF => {
                            // any number of 0xFF in a row is allowed and should be ignored
                            current_byte = val_iter.next();
                            current_byte_ = current_byte.unwrap();
                            continue;
                        }
                        _ => {
                            valid = false;
                            break;
                        }
                    }
                }
                _ => {
                    valid = false;
                    break;
                }
            }

            last_byte = val_iter.next();
            current_byte = val_iter.next();
        }

        let result = JFIFContainer { segments };

        result
    }
}

impl GeneralSegment {
    pub fn get_size(&self) -> usize {
        self.data.len() + 2
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
    fn get_marker(&self) -> Option<[u8; 2]>;
}

impl ToBytes for GeneralSegment {
    fn get_marker(&self) -> Option<[u8; 2]> {
        None
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let data_len = self.get_size();
        let len_bytes = usize_to_be(data_len);

        result.push(len_bytes[2]);
        result.push(len_bytes[3]);
        result.extend_from_slice(&self.data[..]);

        result
    }
}

impl ToBytes for JFIFSegment {
    fn get_marker(&self) -> Option<[u8; 2]> {
        match self {
            JFIFSegment::SOI => Some([0xFF, 0xD8]),
            JFIFSegment::SOF0(_) => Some([0xFF, 0xC0]),
            JFIFSegment::SOF2(_) => Some([0xFF, 0xC2]),
            JFIFSegment::DHT(_) => Some([0xFF, 0xC4]),
            JFIFSegment::DQT(_) => Some([0xFF, 0xDB]),
            JFIFSegment::DRI(_) => Some([0xFF, 0xDD]),
            JFIFSegment::SOS(_) => Some([0xFF, 0xDA]),
            JFIFSegment::COM(_) => Some([0xFF, 0xFE]),
            JFIFSegment::EOI => Some([0xFF, 0xD9]),
            JFIFSegment::APP(seg, _) => Some([0xFF, 0xE0 | (0xF & *seg)]),
            JFIFSegment::RST(seg) => Some([0xFF, 0xD0 | (0b111 & seg)]),
            JFIFSegment::IMGDATA(_) => None,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            JFIFSegment::SOI => Vec::from(self.get_marker().unwrap()),
            JFIFSegment::EOI => Vec::from(self.get_marker().unwrap()),
            JFIFSegment::RST(_) => Vec::from(self.get_marker().unwrap()),
            JFIFSegment::IMGDATA(vec) => vec.clone(),
            JFIFSegment::DRI(payload) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.push(0x00);
                result.push(0x04);
                result.extend(payload);
                result
            }
            JFIFSegment::SOF0(seg) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.extend_from_slice(&seg.to_bytes());
                result
            }
            JFIFSegment::SOF2(seg) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.extend_from_slice(&seg.to_bytes());
                result
            }
            JFIFSegment::SOS(seg) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.extend_from_slice(&seg.to_bytes());
                result
            }
            JFIFSegment::COM(seg) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.extend_from_slice(&seg.to_bytes());
                result
            }
            JFIFSegment::DQT(seg) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.extend_from_slice(&seg.to_bytes());
                result
            }
            JFIFSegment::DHT(seg) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.extend_from_slice(&seg.to_bytes());
                result
            }
            JFIFSegment::APP(_, seg) => {
                let mut result = Vec::from(self.get_marker().unwrap());
                result.extend_from_slice(&seg.to_bytes());
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general_segment_len() {
        let segment = GeneralSegment {
            data: vec![0x0F, 0x0D, 0x44],
        };
        let segment_len = segment.get_size();

        assert_eq!(segment_len, 5);
    }

    #[test]
    fn general_segment_marker_return_none() {
        let segment = GeneralSegment {
            data: vec![0x0F, 0x0D, 0x44],
        };
        let marker = segment.get_marker();

        assert_eq!(marker, None);
    }

    #[test]
    fn general_segment_bytes() {
        let segment = GeneralSegment {
            data: vec![0x0F, 0x0D, 0x44],
        };
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0x00, 0x05, 0x0F, 0x0D, 0x44]);
    }

    #[test]
    fn segment_soi_marker() {
        let segment = JFIFSegment::SOI;
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xD8]));
    }

    #[test]
    fn segment_soi_bytes() {
        let segment = JFIFSegment::SOI;
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xD8]);
    }

    #[test]
    fn segment_eoi_marker() {
        let segment = JFIFSegment::EOI;
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xD9]));
    }

    #[test]
    fn segment_eoi_bytes() {
        let segment = JFIFSegment::EOI;
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xD9]);
    }

    #[test]
    fn segment_rst_marker() {
        for i in 0..8 {
            let segment = JFIFSegment::RST(i);
            let marker = segment.get_marker();

            assert_eq!(marker, Some([0xFF, 0xD0 + i]));
        }
    }

    #[test]
    fn segment_rst_bytes() {
        for i in 0..8 {
            let segment = JFIFSegment::RST(i);
            let bytes = segment.to_bytes();

            assert_eq!(bytes, [0xFF, 0xD0 + i]);
        }
    }

    #[test]
    fn segment_dri_marker() {
        let segment = JFIFSegment::DRI([0x04, 0x02]);
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xDD]));
    }

    #[test]
    fn segment_dri_bytes() {
        let segment = JFIFSegment::DRI([0x04, 0x02]);
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xDD, 0x00, 0x04, 0x04, 0x02]);
    }

    #[test]
    fn segment_img_marker() {
        let segment = JFIFSegment::IMGDATA(vec![0x0A, 0x0B]);
        let marker = segment.get_marker();

        assert_eq!(marker, None);
    }

    #[test]
    fn segment_img_bytes() {
        let segment = JFIFSegment::IMGDATA(vec![0x0A, 0x0B]);
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0x0A, 0x0B]);
    }

    #[test]
    fn segment_app_marker() {
        for i in 0..0xF {
            let segment = JFIFSegment::APP(
                i,
                GeneralSegment {
                    data: vec![0x02, 0x04],
                },
            );
            let marker = segment.get_marker();

            assert_eq!(marker, Some([0xFF, 0xE0 + i]));
        }
    }

    #[test]
    fn segment_app_bytes() {
        let segment = JFIFSegment::APP(
            0,
            GeneralSegment {
                data: vec![0x02, 0x04],
            },
        );
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xE0, 0x00, 0x04, 0x02, 0x04]);
    }

    #[test]
    fn segment_com_marker() {
        let segment = JFIFSegment::COM(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xFE]));
    }

    #[test]
    fn segment_com_bytes() {
        let segment = JFIFSegment::COM(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xFE, 0x00, 0x04, 0x02, 0x04]);
    }

    #[test]
    fn segment_sof0_marker() {
        let segment = JFIFSegment::SOF0(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xC0]));
    }

    #[test]
    fn segment_sof0_bytes() {
        let segment = JFIFSegment::SOF0(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xC0, 0x00, 0x04, 0x02, 0x04]);
    }

    #[test]
    fn segment_sof2_marker() {
        let segment = JFIFSegment::SOF2(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xC2]));
    }

    #[test]
    fn segment_sof2_bytes() {
        let segment = JFIFSegment::SOF2(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xC2, 0x00, 0x04, 0x02, 0x04]);
    }

    #[test]
    fn segment_dht_marker() {
        let segment = JFIFSegment::DHT(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xC4]));
    }

    #[test]
    fn segment_dht_bytes() {
        let segment = JFIFSegment::DHT(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xC4, 0x00, 0x04, 0x02, 0x04]);
    }

    #[test]
    fn segment_dqt_marker() {
        let segment = JFIFSegment::DQT(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xDB]));
    }

    #[test]
    fn segment_dqt_bytes() {
        let segment = JFIFSegment::DQT(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xDB, 0x00, 0x04, 0x02, 0x04]);
    }

    #[test]
    fn segment_sos_marker() {
        let segment = JFIFSegment::SOS(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let marker = segment.get_marker();

        assert_eq!(marker, Some([0xFF, 0xDA]));
    }

    #[test]
    fn segment_sos_bytes() {
        let segment = JFIFSegment::SOS(GeneralSegment {
            data: vec![0x02, 0x04],
        });
        let bytes = segment.to_bytes();

        assert_eq!(bytes, [0xFF, 0xDA, 0x00, 0x04, 0x02, 0x04]);
    }
}