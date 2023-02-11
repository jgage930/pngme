mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Cli;
use clap::Parser;

use args::{DecodeArgs, EncodeArgs, PngMeArgs, PrintArgs, RemoveArgs};
use chunk::Chunk;
use chunk_type::ChunkType;
use png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.arg {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print_chunks(args),
    }
}

fn encode(args: EncodeArgs) -> Result<()> {
    println!("Encoding");
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    println!("Decoding");
    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    println!("Removing");
    Ok(())
}

fn print_chunks(args: PrintArgs) -> Result<()> {
    println!("Printing");
    Ok(())
}
