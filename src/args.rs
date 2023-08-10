use std::path::PathBuf;

use clap::Parser;

/// Command-line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input YAML file.
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Output binary file.
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}
