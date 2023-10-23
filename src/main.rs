use chrono::{Duration, NaiveDateTime};
use clap::{Parser, Subcommand};

// name: Task name
// due: 2022-3-3 34:44:44
// recurence_type: strict, cooldown
// recurence: 2d
// completed: [datetime, datetime, datetime]
// traking: [[datetime,datetime], [datetime,datetime]]
// projects: []

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        #[arg(short, long, default_value_t = String::from("Inbox"))]
        project: String,
        #[arg(short, long)]
        schedule: Option<NaiveDateTime>,
        #[arg(short = 't', long, default_value_t = String::from("strict"), value_parser = clap::builder::PossibleValuesParser::new(["strict", "cooldown"]))]
        recurrence_type: String,
        #[arg(short = 'u', long, default_value_t = String::from("d"), value_parser = clap::builder::PossibleValuesParser::new(["d", "w"]))]
        recurrence_unit: String,
        #[arg(short, long)]
        recurrence: Option<i64>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add {
            name,
            project,
            schedule,
            recurrence_type,
            recurrence_unit,
            recurrence,
        }) => {
            println!("{name}, {project}, {recurrence_type}, {recurrence_unit}")
        }
        None => {}
    }
}
