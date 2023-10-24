extern crate dirs;
use crate::task::Task;

use chrono::{Duration, NaiveDateTime};
use clap::{Parser, Subcommand};
use uuid::Uuid;
#[derive(clap::Args)]
pub struct AddArgs {
    name: String,
    // #[arg(short('i'), long)]
    // priority: Option<char>,
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
}
fn get_data_local_dir() -> Option<std::path::PathBuf> {
    match dirs::data_local_dir() {
        Some(data_local_dir) => {
            let dir = data_local_dir.join("mashiro-task");
            let _ = std::fs::create_dir(&dir);
            Some(dir)
        }
        None => None,
    }
}
impl AddArgs {
    pub fn run(&self) {
        println!(
            "{}, {}, {}, {}",
            self.name, self.project, self.recurrence_type, self.recurrence_unit
        );
        let mut uuid = Uuid::new_v4();
        let task_path = loop {
            let dir = match get_data_local_dir() {
                Some(dir) => dir.join(format!("{uuid}.json")),
                None => std::process::exit(1),
            };
            if !dir.exists() {
                break dir;
            }
            uuid = Uuid::new_v4();
        };
        let t = Task {
            name: self.name.to_owned(),
            project: self.project.to_owned(),
            schedule: self.schedule.to_owned(),
            recurrence_type: Some(self.recurrence_type.to_owned()),
            recurrence_unit: Some(self.recurrence_unit.to_owned()),
            recurrence: self.recurrence.to_owned(),
            completions: Vec::new(),
            intervals: Vec::new(),
        };
        match serde_json::to_string_pretty(&t) {
            Ok(j) => {
                if let Err(err) = std::fs::write(&task_path, j) {
                    eprintln!("Error writing to file: {:?}", err);
                } else {
                    println!("Content saved to {}", task_path.display());
                }
            }
            _ => {}
        };
    }
}
