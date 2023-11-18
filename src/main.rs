extern crate dirs;
mod add;
mod now;
mod task;
mod paths;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author("Niko Honu"), version("0.1"), about("My personal to-do list app."), long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a task
    Add(add::AddArgs),
    Now(now::NowArgs),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add(cmd)) => cmd.run(),
        Some(Commands::Now(cmd)) => cmd.run(),
        _ => now::NowArgs {}.run(),
    };
}
