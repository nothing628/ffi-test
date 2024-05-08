use anyhow::Result;
use ffi_test::jpeg::container::JFIFContainer;
use ffi_test::jpeg::container::JFIFSegment;
use ffi_test::jpeg::container::ToBytes;
use std::fs::read;

pub fn main() -> Result<()> {
    let jpeg_test = read("./test.jpeg")?;
    println!("Hi");
    println!("File length : {}", jpeg_test.len());

    let container = JFIFContainer::try_from(&jpeg_test);

    match container {
        Ok(container) => {
            let segments = container.get_segments();
            println!("Segment count : {}", segments.len());
            for segment in segments {
                match segment {
                    JFIFSegment::SOI => println!("Segment SOI, Len : {}", JFIFSegment::SOI.to_bytes().len()),
                    JFIFSegment::EOI => println!("Segment EOI, Len : {}", JFIFSegment::EOI.to_bytes().len()),
                    JFIFSegment::DQT(_) => println!("Segment DQT, Len : {}", segment.to_bytes().len()),
                    JFIFSegment::DHT(_) => println!("Segment DHT, Len : {}", segment.to_bytes().len()),
                    JFIFSegment::SOF2(_) => println!("Segment SOF2, Len : {}", segment.to_bytes().len()),
                    JFIFSegment::SOS(_) => println!("Segment SOS, Len : {}", segment.to_bytes().len()),
                    JFIFSegment::IMGDATA(seg) => println!("Segment IMGDATA, Len : {}", seg.len()),
                    JFIFSegment::APP(app, _) => println!("Segment APP{}, Len : {}", app, segment.to_bytes().len()),
                    _ => println!("Unknown segment {:?}", segment),
                }
            }

            let convert_back = Vec::from(container);

            println!("Output length : {}", convert_back.len());
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
