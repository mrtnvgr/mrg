use super::writer::Writer;
use crate::{Point, Track};
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

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
                buffer.write_int(point.x)?;
                buffer.write_int(point.y)?;
            } else {
                buffer.write_byte(i8::try_from(x)?)?;
                buffer.write_byte(i8::try_from(y)?)?;
            }

            prev_point = point;
        }

        Ok(())
    }
}
