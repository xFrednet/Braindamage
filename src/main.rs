use clap::{Args, Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "braindamage")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(RunArgs),
}

#[derive(Args, Debug)]
struct RunArgs {
    // FIXME: Would be cool, if this wouldn't require the `-f` flag
    /// The file to run
    #[arg(short, long, required = true)]
    file: Option<PathBuf>,

    // FIXME: This should take a string, to allow units like
    // 1kb or 10mb
    /// The amount of memory which should be provided by the interpreter
    #[arg(short, long, default_value = "1000")]
    memory: usize,

    /// The start position of the memory head.
    #[arg(long, default_value = "100")]
    start: usize,

    // FIXME: Make default value dependent on build profile.
    /// Enable Debug Mode
    #[arg(short, long, default_value = "true")]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("Output: {:#?}", cli);
}
