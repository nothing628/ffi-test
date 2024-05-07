use crate::jpeg::container::GeneralSegment;
use crate::jpeg::container::ToBytes;
use crate::jpeg::container::JFIFContainer;
use crate::jpeg::container::JFIFSegment;
use crate::file_joiner::be_to_usize;
use core::iter::Peekable;
use core::slice::Iter;
use anyhow::Result;
use anyhow::anyhow;

fn read_segments(iter: &mut Box<&mut Peekable<Iter<u8>>>) -> Result<Vec<u8>> {
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

            if *last_byte_ != 0xFF {
                valid = false;
                break;
            }

            match last_byte_ {
                0xFF => {
                    match current_byte_ {
                        0xC0 => {
                            let mut iter = Box::new(&mut val_iter);
                            match read_segments(&mut iter) {
                                Ok(data) => {
                                    segments.push(JFIFSegment::SOF0(GeneralSegment::new(data)));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
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
                                    valid = false;
                                    break;
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
                            let mut iter = Box::new(&mut val_iter);
                            match read_segments(&mut iter) {
                                Ok(data) => {
                                    segments.push(JFIFSegment::DQT(GeneralSegment::new(data)));
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
                            let mut iter = Box::new(&mut val_iter);
                            match read_segments(&mut iter) {
                                Ok(data) => {
                                    segments
                                        .push(JFIFSegment::APP(app_num, GeneralSegment::new(data)));
                                }
                                Err(_) => {
                                    valid = false;
                                    break;
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

        let result = JFIFContainer::new(segments);

        result
    }
}
