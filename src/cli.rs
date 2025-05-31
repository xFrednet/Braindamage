use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    pub file: PathBuf,
    #[arg(short, long, default_value = "a.bf")]
    pub output: PathBuf,
}

impl Cli {
    pub(crate) fn from_args() -> Self {
        Self::parse()
    }
}
