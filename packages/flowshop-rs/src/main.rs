use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(SubCommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
    }
}

pub fn main() {
    let cli = Cli::parse();

}
