use super::writer::Writer;
use crate::Difficulty;
use anyhow::Result;
use std::iter::zip;

impl Difficulty {
    pub(super) fn to_bytes(
        &self,
        buffer: &mut Vec<u8>,
        track_offsets: &mut Vec<Vec<usize>>,
    ) -> Result<()> {
        let mut offsets = Vec::new();

        for track in &self.tracks {
            offsets.push(buffer.len());
            track.to_bytes(buffer)?;
        }

        track_offsets.push(offsets);

        Ok(())
    }

    pub(super) fn generate_header(
        &self,
        offsets: &Vec<usize>,
        header_size: usize,
    ) -> Result<Vec<u8>> {
        let mut header: Vec<u8> = Vec::new();

        let track_count: i32 = self.tracks.len().try_into()?;
        header.write_int(track_count)?;

        for (offset, track) in zip(offsets, &self.tracks) {
            let offset: i32 = (offset + header_size).try_into()?;
            header.write_int(offset)?;

            header.extend(track.name.as_bytes());

            header.push(0x00);
        }

        Ok(header)
    }
}
