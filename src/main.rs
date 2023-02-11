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

use std::fs;

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
    let bytes = fs::read(args.file_path)?;
    let mut png = Png::try_from(bytes.as_slice())?;

    let chunk = Chunk::new(
        args.chunk_type,
        args.message.as_bytes().iter().cloned().collect(),
    );

    png.append_chunk(chunk);

    fs::write(args.output_file, png.as_bytes())?;

    println!("Encoded Message");

    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let bytes = fs::read(args.file_path)?;
    let png = Png::try_from(bytes.as_slice())?;

    let chunk_type_name = String::from_utf8(args.chunk_type.bytes().iter().cloned().collect())?;

    match png.chunk_by_type(&chunk_type_name) {
        Some(chunk) => {
            let chunk_data = chunk.data();

            let message = String::from_utf8(chunk_data.iter().cloned().collect())?;
            println!("Decoded Message: {}", message);
        }
        None => {
            println!("Could not find chunk");
        }
    }

    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    let mut png = Png::try_from(bytes.as_slice())?;

    let chunk_type_name = String::from_utf8(args.chunk_type.bytes().iter().cloned().collect())?;
    png.remove_chunk(&chunk_type_name)?;

    fs::write(&args.file_path, png.as_bytes())?;

    println!("Chunk Removed");

    Ok(())
}

fn print_chunks(args: PrintArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(bytes.as_slice())?;

    for chunk in png.chunks().iter() {
        println!("{:?}", chunk);
    }
    Ok(())
}
