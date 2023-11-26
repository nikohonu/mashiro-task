extern crate dirs;

use crate::task::Task;

use chrono::{Local, NaiveDateTime};

use uuid::Uuid;
#[derive(clap::Args, Debug)]
pub struct AddArgs {
    name: String,
    #[arg(short, long, default_value_t = String::from("Inbox"))]
    project: String,
    #[arg(short, long)]
    schedule: Option<NaiveDateTime>,
    #[arg(short = 't', long, default_value_t = String::from("c"), value_parser = clap::builder::PossibleValuesParser::new(["+", "++", ".+", "c"]))]
    recurrence_type: String,
    #[arg(short = 'u', long, default_value_t = String::from("d"), value_parser = clap::builder::PossibleValuesParser::new(["d", "w"]))]
    recurrence_unit: String,
    #[arg(short, long, default_value_t = 1)]
    recurrence: u64,
    #[arg(short = 'm', long, default_value_t = false)]
    required: bool,
    #[arg(long)]
    required_task: Option<String>,
    // #[arg(short, long, default_value_t = false)]
    // json: bool,
}
impl AddArgs {
    pub fn run(&self) {
        let schedule = if let Some(s) = self.schedule {
            s
        } else {
            Local::now().naive_local()
        };
        let t = Task {
            uuid: Uuid::new_v4().to_string(),
            created: Local::now().naive_local(),
            id: Task::get_new_id(),
            name: self.name.to_owned(),
            project: self.project.to_owned(),
            schedule,
            recurrence_type: self.recurrence_type.to_owned(),
            recurrence_unit: self.recurrence_unit.to_owned(),
            recurrence: self.recurrence.to_owned(),
            required: self.required,
            required_task: self.required_task.to_owned(),
            now_datetime: None,
            times_completed: 0,
        };
        Task::append(&t);
        println!("{}", serde_json::to_string_pretty(&t).unwrap_or_default())
    }
}
