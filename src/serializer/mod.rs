mod difficulty;
mod point;
mod track;
mod writer;

#[cfg(test)]
mod tests;

use crate::{Difficulty, Mrg};
use anyhow::Result;
use std::iter::zip;

impl Mrg {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut body_buffer: Vec<u8> = Vec::new();
        let mut track_offsets: Vec<Vec<usize>> = Vec::new();

        // Body
        for diff in self.iter_diffs() {
            diff.to_bytes(&mut body_buffer, &mut track_offsets)?;
        }

        // Header
        let mut header_buffer: Vec<u8> = Vec::new();
        let header_size = self.calculate_header_size();

        for (offsets, track) in zip(track_offsets, self.iter_diffs()) {
            header_buffer.extend(track.generate_header(&offsets, header_size)?);
        }

        header_buffer.extend(body_buffer);
        Ok(header_buffer)
    }

    fn calculate_header_size(&self) -> usize {
        let mut size = 0;

        for diff in self.iter_diffs() {
            // Tracks count
            size += 4;

            for track in &diff.tracks {
                // Track offset
                size += 4;

                // Track name
                size += track.name.as_bytes().len();

                // Null byte
                size += 1;
            }
        }

        size
    }

    const fn iter_diffs(&self) -> [&Difficulty; 3] {
        [&self.easy, &self.normal, &self.hard]
    }
}
