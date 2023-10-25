use std::str::FromStr;
use crate::chunk::Chunk;

mod chunk_type;
mod chunk;
mod png;

#[macro_use]
extern crate derive_error;




pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // let chunk = testing_chunk();


    todo!()
}
