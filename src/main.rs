use clap::{Args, Parser, Subcommand};

use std::path::{Path, PathBuf};

mod interpreter;

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
    file: PathBuf,

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

    match cli.command {
        Commands::Run(args) => {
            let program = load_file(&args.file).unwrap();
            let mut inter = interpreter::Interpreter::new(&program, args.memory, args.debug, args.start);
            inter.run().unwrap();
        },
    }
}

fn load_file(file: &Path) -> Result<String, String> {
    let file_name = file.to_string_lossy();
    let data = std::fs::read(file).map_err(|err| format!(r#"Unable to open "{file_name}": {err:#?}"#))?;

    String::from_utf8(data).map_err(|err| format!(r#"File "{file_name}" is not valid UTF8: {err:#?}"#))
}
