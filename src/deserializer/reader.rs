#![allow(clippy::indexing_slicing)]

use anyhow::Result;
use endianness::{read_i16, read_i32, ByteOrder};

pub struct Reader<'re> {
    bytes: &'re [u8],
    offset: usize,
}

impl<'re> Reader<'re> {
    pub const fn new(bytes: &'re [u8]) -> Self {
        let offset = 0;
        Self { bytes, offset }
    }

    pub const fn clone_from_offset(&self, offset: usize) -> Option<Self> {
        if offset > self.bytes.len() {
            None
        } else {
            Some(Self {
                bytes: self.bytes,
                offset,
            })
        }
    }

    pub fn read_int(&mut self) -> Result<i32> {
        let value = read_i32(&self.bytes[self.offset..], ByteOrder::BigEndian)?;
        self.offset += std::mem::size_of::<i32>();
        Ok(value)
    }

    pub fn read_short(&mut self) -> Result<i16> {
        let value = read_i16(&self.bytes[self.offset..], ByteOrder::BigEndian)?;
        self.offset += std::mem::size_of::<i16>();
        Ok(value)
    }

    pub fn read_byte(&mut self) -> i8 {
        let value = self.bytes[self.offset];
        self.offset += std::mem::size_of::<i8>();
        value as i8
    }
}
