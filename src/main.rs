use std::str::FromStr;

use crate::chunk_type::{
    ChunkType,
};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let chunk = ChunkType::from_str("RuSt").unwrap();

}