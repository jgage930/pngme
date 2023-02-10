use clap::Parser;
use std::path::PathBuf;

use crate::chunk_type::ChunkType;

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct EncodeArgs {
    #[arg(short, long, help = "Path to input file")]
    file_path: PathBuf,

    #[arg(short, long)]
    chunk_type: ChunkType,

    #[arg(short, long)]
    message: String,

    #[arg(short, long)]
    output_file: PathBuf,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DecodeArgs {
    #[arg(short, long, help = "Path to input file")]
    file_path: PathBuf,

    #[arg(short, long)]
    chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RemoveArgs {
    #[arg(short, long, help = "Path to input file")]
    file_path: PathBuf,

    #[arg(short, long)]
    chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PrintArgs {
    #[arg(short, long, help = "Path to input file")]
    file_path: PathBuf,
}
