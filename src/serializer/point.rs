use super::writer::Writer;
use crate::Point;
use anyhow::Result;

impl Point {
    pub(super) fn to_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_int(self.x)?;
        buffer.write_int(self.y)?;
        Ok(())
    }
}
