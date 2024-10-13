mod commands;
mod datetime;
mod recurrence;
mod task;
mod tasks;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use commands::Commands;
use tasks::Tasks;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Now(args)) => args.run()?,
        Some(Commands::Done(args)) => args.run()?,
        Some(Commands::Schedule(args)) => args.run()?,
        Some(Commands::Other(args)) => args.run()?,
        _ => println!("no subcommand"),
    };
    Ok({})
}
