mod difficulty;
mod reader;
mod track;

#[cfg(test)]
mod tests;

use self::reader::Reader;
use crate::{Difficulty, Mrg};

impl Mrg {
    pub fn from_bytes(bytes: Vec<u8>) -> anyhow::Result<Self> {
        let mut reader = Reader::new(bytes);
        let easy = Difficulty::from_reader(&mut reader)?;
        let normal = Difficulty::from_reader(&mut reader)?;
        let hard = Difficulty::from_reader(&mut reader)?;
        Ok(Self { easy, normal, hard })
    }
}
