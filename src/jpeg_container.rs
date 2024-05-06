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
    RST(GeneralSegment),
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
            JFIFSegment::RST(seg) => seg.marker,
            JFIFSegment::IMGDATA(_) => [0x00, 0x00],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            JFIFSegment::SOI => Vec::from(JFIFSegment::SOI.get_marker()),
            JFIFSegment::EOI => Vec::from(JFIFSegment::EOI.get_marker()),
            JFIFSegment::IMGDATA(vec) => vec.clone(),
            JFIFSegment::DRI(payload) => Vec::from(payload),
        }
    }
}
