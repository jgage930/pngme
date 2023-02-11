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
    file_path: PathBuf,

    chunk_type: ChunkType,

    message: String,

    output_file: PathBuf,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    file_path: PathBuf,

    chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    file_path: PathBuf,

    chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    file_path: PathBuf,
}
