use byteorder::{BigEndian, WriteBytesExt};
use std::io::{self, Result};

pub trait Writer: io::Write {
    fn write_int(&mut self, value: i32) -> Result<()> {
        self.write_i32::<BigEndian>(value)
    }

    fn write_short(&mut self, value: i16) -> Result<()> {
        self.write_i16::<BigEndian>(value)
    }

    fn write_byte(&mut self, value: i8) -> Result<()> {
        self.write_i8(value)
    }
}

impl<W: io::Write + ?Sized> Writer for W {}
