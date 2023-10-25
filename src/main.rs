// mod args;
use std::str::FromStr;
use crate::chunk::Chunk;

mod chunk_type;
mod chunk;
// mod commands;
// mod png;

#[macro_use]
extern crate derive_error;


fn testing_chunk() -> Chunk {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();

    Chunk::try_from(chunk_data.as_ref()).unwrap()
}

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chunk = testing_chunk();


    todo!()
}
