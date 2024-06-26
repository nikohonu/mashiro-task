extern crate dirs;

mod add;
mod completion;
mod r#do;
mod interval;
mod list;
mod now;
mod paths;
mod recurrence;
mod regenerate_ids;
mod remove;
mod task;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author("Niko Honu"), version("0.5.0"), about("A cli to-do list app that focuses on recurring tasks and displays only three current tasks."), long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    full: bool,
    #[arg(short, long, default_value_t = false)]
    random: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add(add::AddArgs),
    Completion(completion::CompletionArgs),
    List(list::ListArgs),
    Now(now::NowArgs),
    Do(r#do::DoArgs),
    Remove(remove::RemoveArgs),
    RegenerateIds(regenerate_ids::RegenerateIdsArgs),
}

fn main() {
    let cli = Cli::parse();
    let _command = Cli::command();
    match &cli.command {
        Some(Commands::Add(cmd)) => cmd.run(),
        Some(Commands::Completion(cmd)) => cmd.run(&mut Cli::command()),
        Some(Commands::List(cmd)) => cmd.run(),
        Some(Commands::Now(cmd)) => cmd.run(),
        Some(Commands::RegenerateIds(cmd)) => cmd.run(),
        Some(Commands::Do(cmd)) => cmd.run(),
        Some(Commands::Remove(cmd)) => cmd.run(),
        _ => now::NowArgs {
            full: cli.full,
            random: cli.random,
        }
        .run(),
    }
}
