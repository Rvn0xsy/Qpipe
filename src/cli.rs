use std::path::PathBuf;
use clap::{Parser};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,

    #[arg(short, long)]
    pub(crate) debug: bool,
}
