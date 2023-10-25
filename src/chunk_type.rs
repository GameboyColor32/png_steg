use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::bail;

use crate::chunk_type::ChunkTypeError::InvalidByteError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4]
}

#[derive(Error, Debug)]
pub enum ChunkTypeError {
    #[error("A valid chunk should be only alphanumeric characters")]
    InvalidByteError,
    // #[error("A valid chunk should be only alphanumeric characters")]
    // InvalidByteError,
}

const ANCILLARY_BIT: usize = 0;
const PUBLIC_BIT: usize = 1;
const RESERVED_BIT: usize = 2;
const SAFE_TO_COPY: usize = 3;

fn is_ascii_alphabetic(c: &u8) -> bool {
    c.is_ascii_alphabetic()
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && self.bytes.iter().all(is_ascii_alphabetic)
    }

    fn is_critical(&self) -> bool {
        self.bytes[ANCILLARY_BIT] & (1 << 5) == 0
    }

    fn is_public(&self) -> bool {
        self.bytes[PUBLIC_BIT] & (1 << 5) == 0
    }

    fn is_reserved_bit_valid(&self) -> bool { self.bytes[RESERVED_BIT] & (1 << 5) == 0 }

    fn is_safe_to_copy(&self) -> bool {
        !self.bytes[SAFE_TO_COPY] & (1 << 5) == 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for v in value {
            if !v.is_ascii_alphabetic() {
                println!("A valid chunk should be only alphanumeric characters");
                bail!(InvalidByteError)
            }
        }
        Ok(Self {bytes: value})
    }
}

impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            println!("Invalid string");
            return Err(());
        }

        let s = Self {bytes: copy_string_to_byte_array(&s)};
        if !s.bytes.iter().all(is_ascii_alphabetic) { return Err(()); }
        Ok(s)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes).unwrap())
    }
}

fn copy_string_to_byte_array(s: &str) -> [u8; 4] {
    let mut byte_array: [u8; 4] = [0; 4];

    for (i, byte) in s.as_bytes().iter().enumerate() { byte_array[i] = *byte; }
    byte_array
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());
        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
