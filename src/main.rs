extern crate dirs;
mod add;
mod completion;
mod regenerate_ids;
mod now;
mod paths;
mod task;



use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author("Niko Honu"), version("0.1"), about("My personal to-do list app."), long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add(add::AddArgs),
    Completion(completion::CompletionArgs),
    Now(now::NowArgs),
    RegenerateIds(regenerate_ids::RegenerateIdsArgs),
}

fn main() {
    let cli = Cli::parse();
    let _command = Cli::command();
    match &cli.command {
        Some(Commands::Add(cmd)) => cmd.run(),
        Some(Commands::Completion(cmd)) => cmd.run(&mut Cli::command()),
        Some(Commands::Now(cmd)) => cmd.run(),
        Some(Commands::RegenerateIds(cmd)) => cmd.run(),
        _ => now::NowArgs {}.run(),
    }
}
