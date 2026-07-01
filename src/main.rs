use clap::Parser;
use sped_to_database::adapters::cli::commands::Cli;

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    if let Err(e) = sped_to_database::adapters::cli::runner::run(cli).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}