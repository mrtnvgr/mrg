#![allow(clippy::indexing_slicing)]

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Result};

pub(super) struct Reader {
    cursor: Cursor<Vec<u8>>,
}

impl Reader {
    pub fn new(bytes: Vec<u8>) -> Self {
        let cursor = Cursor::new(bytes);
        Self { cursor }
    }

    pub fn clone_from_offset(&self, offset: usize) -> Self {
        let bytes = self.cursor.get_ref().clone();
        let mut cursor = Cursor::new(bytes);
        cursor.set_position(offset as u64);
        Self { cursor }
    }

    pub fn read_int(&mut self) -> Result<i32> {
        self.cursor.read_i32::<BigEndian>()
    }

    pub fn read_short(&mut self) -> Result<i16> {
        self.cursor.read_i16::<BigEndian>()
    }

    pub fn read_byte(&mut self) -> Result<i8> {
        self.cursor.read_i8()
    }
}
