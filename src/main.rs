use std::str::FromStr;
use crate::png::Png;
use clap::Parser;

use std::fs::{read as read_file, File};
use std::io::Write;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

mod chunk_type;
mod chunk;
mod png;
mod commands;

#[macro_use]
extern crate derive_error;

use crate::commands::Args;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();
    // let chunk = testing_chunk();

    // println!("Hello {}!", args.command_type);


    let file = read_file(args.path)?;
    let mut png = Png::try_from(file.as_slice()).unwrap();
    let chunk_type = ChunkType::from_str(args.chunk_name.as_str()).unwrap();

    println!("png: {}", png);

    match &args.command[..] {
        "encode" => {
            let message = args.secret.as_ref().expect("Fourth argument is required for 'encode'");
            println!("Command: encode, message: {}", &message);
            png.append_chunk(Chunk::new(chunk_type, message.clone().into_bytes()));
            let mut output =
                File::create("./output.png").map_err(|e| eprintln!("error creating file")).expect("error creating file");
            output.write(png.as_bytes().as_slice());
        }
        "decode" => {
            println!("Command: {}", args.command);
            println!("chunk: {}", png.chunk_by_type(args.chunk_name.as_str()).unwrap());
        },
        _ => {eprintln!("unrecognized command: {}",args.command)}
    }


    Ok(())
}
