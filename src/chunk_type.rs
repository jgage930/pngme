use std::str::{
    FromStr,
};
use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct ChunkType {
    pub bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = fmt::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, fmt::Error> {
        for byte in value.iter() {
            if byte > &255 {
                return Err(fmt::Error)
            }
        }
        Ok(Self { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, fmt::Error> {
        let chars = String::from(s).into_bytes();

        let chunk = ChunkType {
            bytes: chars.try_into().unwrap(),
        };

        if chunk.is_valid() {
            return Ok(chunk);
        } else {
            return Err(fmt::Error);
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!();
    }
    
}


impl ChunkType {

    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }


    pub fn is_valid(&self) -> bool {
        //! Need to seperate out the checking in range and checking reversed bit
        let mut flag = true;
        for byte in self.bytes.iter() {
            if *byte >= 65 && *byte <= 90 {
                flag = true;
            }

            if *byte >= 97 && *byte <= 122 {
                flag = true;
            }
        }

        // if !self.is_reserved_bit_valid() {
        //     flag = false;
        // }

        flag
    }

    fn is_critical(&self) -> bool {
        u8::is_ascii_uppercase(&self.bytes[0])
    }

    fn is_public(&self) -> bool {
        u8::is_ascii_uppercase(&self.bytes[1]) 
    }
    
    fn is_reserved_bit_valid(&self) -> bool {
        u8::is_ascii_uppercase(&self.bytes[2])
    }

    fn is_safe_to_copy(&self) -> bool {
        u8::is_ascii_lowercase(&self.bytes[3])
    }

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
