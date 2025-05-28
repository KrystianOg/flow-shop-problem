use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Generate(Generate),
}

#[derive(Args)]
struct Generate {
    string: Option<String>,
}

pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate(args)) => {}
        None => {}
    }
}
