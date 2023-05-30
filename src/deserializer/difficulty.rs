use super::reader::Reader;
use crate::{Difficulty, Track};
use anyhow::Result;

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

            let name = String::from_utf8(name)?;
            tracks.push(Track::new(reader, name, offset.try_into()?)?);
        }

        Ok(Self { tracks })
    }
}
