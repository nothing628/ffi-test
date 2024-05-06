use crate::file_joiner::usize_to_be;

pub struct GeneralSegment {
    marker: [u8; 2],
    data: Vec<u8>,
}

pub enum JFIFSegment {
    SOI,
    SOF0(GeneralSegment),
    SOF2(GeneralSegment),
    DHT(GeneralSegment),
    DQT(GeneralSegment),
    DRI([u8; 4]),
    SOS(GeneralSegment),
    RST(u8),
    APP(GeneralSegment),
    COM(GeneralSegment),
    IMGDATA(Vec<u8>),
    EOI,
}

pub struct JFIFContainer {
    segments: Vec<JFIFSegment>,
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
    fn get_marker(&self) -> [u8; 2];
}

impl ToBytes for GeneralSegment {
    fn get_marker(&self) -> [u8; 2] {
        self.marker
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let data_len = self.data.len() + 2;
        let len_bytes = usize_to_be(data_len);

        result.push(len_bytes[2]);
        result.push(len_bytes[3]);
        result.extend_from_slice(&self.data[..]);

        result
    }
}

impl ToBytes for JFIFSegment {
    fn get_marker(&self) -> [u8; 2] {
        match self {
            JFIFSegment::SOI => [0xFF, 0xD8],
            JFIFSegment::SOF0(_) => [0xFF, 0xC0],
            JFIFSegment::SOF2(_) => [0xFF, 0xC2],
            JFIFSegment::DHT(_) => [0xFF, 0xC4],
            JFIFSegment::DQT(_) => [0xFF, 0xDB],
            JFIFSegment::DRI(_) => [0xFF, 0xDD],
            JFIFSegment::SOS(_) => [0xFF, 0xDA],
            JFIFSegment::COM(_) => [0xFF, 0xFE],
            JFIFSegment::EOI => [0xFF, 0xD9],
            JFIFSegment::APP(seg) => seg.marker,
            JFIFSegment::RST(seg) => [0xFF, 0xD0 | (0b111 & seg)],
            JFIFSegment::IMGDATA(_) => [0x00, 0x00],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            JFIFSegment::SOI => Vec::from(self.get_marker()),
            JFIFSegment::EOI => Vec::from(self.get_marker()),
            JFIFSegment::RST(_) => Vec::from(self.get_marker()),
            JFIFSegment::IMGDATA(vec) => vec.clone(),
            JFIFSegment::DRI(payload) => Vec::from(payload),
            JFIFSegment::SOF0(seg) => {
                let mut result = Vec::from(self.get_marker());
                result.extend_from_slice(&seg.to_bytes());
                result
            },
            JFIFSegment::SOF2(seg) => {
                let mut result = Vec::from(self.get_marker());
                result.extend_from_slice(&seg.to_bytes());
                result
            },
            JFIFSegment::SOS(seg) => {
                let mut result = Vec::from(self.get_marker());
                result.extend_from_slice(&seg.to_bytes());
                result
            },
            JFIFSegment::COM(seg) => {
                let mut result = Vec::from(self.get_marker());
                result.extend_from_slice(&seg.to_bytes());
                result
            },
            JFIFSegment::DQT(seg) => {
                let mut result = Vec::from(self.get_marker());
                result.extend_from_slice(&seg.to_bytes());
                result
            },
            JFIFSegment::DHT(seg) => {
                let mut result = Vec::from(self.get_marker());
                result.extend_from_slice(&seg.to_bytes());
                result
            },
            JFIFSegment::APP(seg) => {
                let mut result = Vec::from(seg.get_marker());
                result.extend_from_slice(&seg.to_bytes());
                result
            },
        }
    }
}
