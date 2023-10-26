use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub command: String,
    //
    pub path: PathBuf,

    pub chunk_name: String,

    pub secret: Option<String>,
}
