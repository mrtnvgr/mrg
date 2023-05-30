mod writer;

#[cfg(test)]
mod tests;

use self::writer::Writer;
use crate::{Difficulty, Mrg, Point, Track};
use anyhow::{anyhow, Result};
use std::{collections::VecDeque, iter::zip};

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

impl Track {
    pub(super) fn to_bytes(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(0x33);

        let undo_magic = |i| (i << 16) >> 3;

        let start = Point::new(undo_magic(self.start.x), undo_magic(self.start.y));
        let finish = Point::new(undo_magic(self.finish.x), undo_magic(self.finish.y));

        start.to_buffer(buffer)?;
        finish.to_buffer(buffer)?;

        let point_count: i16 = self.points.len().try_into()?;
        buffer.write_short(point_count)?;

        let mut points = VecDeque::new();
        points.extend(&self.points);

        let mut prev_point = points
            .pop_front()
            .ok_or_else(|| anyhow!("Track is empty"))?;

        prev_point.to_buffer(buffer)?;

        for point in points {
            let x = point.x - prev_point.x;
            let y = point.y - prev_point.y;

            if i8::try_from(x).is_err() || i8::try_from(y).is_err() {
                buffer.write_byte(-1)?;
                buffer.write_int(x)?;
                buffer.write_int(y)?;
            } else {
                buffer.write_byte(i8::try_from(x)?)?;
                buffer.write_byte(i8::try_from(y)?)?;
            }

            prev_point = point;
        }

        Ok(())
    }
}

impl Point {
    fn to_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_int(self.x)?;
        buffer.write_int(self.y)?;
        Ok(())
    }
}
