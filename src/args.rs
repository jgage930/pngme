use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::chunk_type::ChunkType;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub arg: PngMeArgs,
}

#[derive(Debug, Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_file: PathBuf,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
