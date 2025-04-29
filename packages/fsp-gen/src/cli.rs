use clap::{Parser, Subcommand};

use crate::generate::{generate_flow_shop, save_to_file};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'm', long)]
    machines: u16,

    #[arg(short = 'j', long)]
    jobs: u16,

    #[arg(short = 'o', long)]
    output_file: Option<String>,

    #[arg(short = 's', long)]
    seed: Option<u32>,
}

pub fn cli() {
    let cli = Cli::parse();

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }
    if cli.debug == 1 {
        println!("Debug mode is on");
    }

    match &cli.command {
        Some(Commands::Generate { list }) => {
            if *list {
                println!("Printing testing lists...");
            }
        }
        None => {
            let (processing_times, seed) = generate_flow_shop(cli.jobs, cli.machines, cli.seed);

            let output_filename: String = match cli.output_file {
                Some(i) => i,
                None => format!("tai{}_{}.txt", cli.jobs, cli.machines),
            };

            let _ = save_to_file(&output_filename, &processing_times, seed);
        }
    }
}
