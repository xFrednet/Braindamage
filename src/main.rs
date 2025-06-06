use clap::{Args, Parser, Subcommand};

use std::path::{Path, PathBuf};

mod interpreter;
pub mod mem;
mod pass;

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

    // FIXME: Using `--emit bf` or somthing like this would be cleaner
    /// The output file for intermediate `.bf` files
    #[arg(short, long)]
    output: Option<PathBuf>,

    // FIXME: This should take a string, to allow units like
    // 1kb or 10mb
    /// The amount of memory which should be provided by the interpreter
    #[arg(short, long, value_parser = clap::value_parser!(mem::MemInfo), default_value = "32")]
    memory: mem::MemInfo,

    // FIXME: Make default value dependent on build profile.
    /// Enable Debug Mode
    #[arg(short, long, default_value = "true")]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => {
            let mut program = load_file(&args.file).unwrap();

            if args.file.extension().map(|ext| ext == "wat-body").unwrap_or_default() {
                program = pass::run_wat_passes(program);
            }

            program = pass::run_base_passes(program);
            if let Some(output) = args.output {
                std::fs::write(output, &program).unwrap();
            }

            let mut inter = interpreter::Interpreter::new(&program, args.memory, args.debug);
            inter.run().unwrap();
        },
    }
}

fn load_file(file: &Path) -> Result<String, String> {
    let file_name = file.to_string_lossy();
    let data = std::fs::read(file).map_err(|err| format!(r#"Unable to open "{file_name}": {err:#?}"#))?;

    String::from_utf8(data).map_err(|err| format!(r#"File "{file_name}" is not valid UTF8: {err:#?}"#))
}
