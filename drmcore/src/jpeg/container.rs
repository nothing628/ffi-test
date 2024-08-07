use crate::file_joiner::usize_to_be;
use core::convert::From;

use super::custom_segment::CustomSegment;

#[derive(Debug)]
pub struct GeneralSegment {
    data: Vec<u8>,
}

impl GeneralSegment {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[derive(Debug)]
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

impl JFIFContainer {
    pub fn new(segments: Vec<JFIFSegment>) -> Self {
        Self { segments }
    }

    pub fn get_segments(&self) -> &Vec<JFIFSegment> {
        &self.segments
    }

    pub fn get_custom_segment(&self) -> Vec<CustomSegment> {
        let mut custom_segments: Vec<CustomSegment> = Vec::new();
        let _ = &self
            .segments
            .iter()
            .filter(|p| {
                let segment = *p;

                return match segment {
                    JFIFSegment::APP(_, _) => {
                        let custom_segment_result = CustomSegment::try_from(segment);

                        return match custom_segment_result {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                    }
                    _ => false,
                };
            })
            .map(|p| -> CustomSegment { CustomSegment::try_from(p).unwrap() })
            .for_each(|f| custom_segments.push(f));

        custom_segments
    }

    pub fn put_custom_segment(&mut self, segment: JFIFSegment) -> Option<usize> {
        let latest_app = self.segments.iter().rposition(|p| {
            return match *p {
                JFIFSegment::APP(_, _) => true,
                _ => false,
            };
        });

        if let Some(latest_app) = latest_app {
            self.segments.insert(latest_app + 1, segment);
            return Some(latest_app + 1);
        }

        None
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
