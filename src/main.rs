use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "braindamage")]
struct Cli {
    file: String,

    #[arg(short, long, default_value = "a.bf")]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Input: {}", cli.file);
    println!("Output: {}", cli.output);
}
