use super::reader::Reader;
use crate::{Difficulty, Track};
use anyhow::Result;
use encoding_rs::WINDOWS_1251;

impl Difficulty {
    pub(super) fn from_reader(reader: &mut Reader) -> Result<Self> {
        let track_count = reader.read_int()?;

        let mut tracks: Vec<Track> = Vec::new();

        for _ in 0..track_count {
            let Ok(offset) = reader.read_int() else { break };

            let mut name: Vec<u8> = Vec::new();
            loop {
                let byte = reader.read_byte()?;

                if byte == 0x00 {
                    break;
                }

                name.extend(byte.to_be_bytes());
            }

            let name = decode_windows_string(name);
            tracks.push(Track::new(reader, name, offset.try_into()?)?);
        }

        Ok(Self { tracks })
    }
}

fn decode_windows_string(bytes: Vec<u8>) -> String {
    let from_1251 = WINDOWS_1251.decode(&bytes);
    let cow = from_1251.0;
    let success = from_1251.2;

    if success {
        String::from_utf8(bytes).unwrap()
    } else {
        String::from(cow)
    }
}
