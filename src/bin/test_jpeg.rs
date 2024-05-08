use ffi_test::jpeg::container::JFIFContainer;
use ffi_test::jpeg::container::JFIFSegment;
use std::fs::read;
use anyhow::Result;

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
                    JFIFSegment::SOI => println!("Segment SOI"),
                    _ => println!("Unknown segment")
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}