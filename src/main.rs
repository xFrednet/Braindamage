#![warn(clippy::pedantic)]

mod cli;
mod fs;
mod pass;

fn main() {
    let cli = cli::Cli::from_args();

    let src = fs::load_file_or_exit(&cli.file);

    let src = pass::run_passes(src);

    fs::write_file_or_exit(&cli.output, &src);
}
