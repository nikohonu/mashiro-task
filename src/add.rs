extern crate dirs;

use crate::task::Task;

use chrono::NaiveDateTime;

use uuid::Uuid;
#[derive(clap::Args, Debug)]
pub struct AddArgs {
    name: String,
    #[arg(short, long, default_value_t = String::from("Inbox"))]
    project: String,
    #[arg(short, long)]
    schedule: Option<NaiveDateTime>,
    #[arg(short, long)]
    deadline: Option<NaiveDateTime>,
    #[arg(short = 't', long, default_value_t = String::from("strict"), value_parser = clap::builder::PossibleValuesParser::new(["strict", "cooldown"]))]
    recurrence_type: String,
    #[arg(short = 'u', long, default_value_t = String::from("d"), value_parser = clap::builder::PossibleValuesParser::new(["d", "w"]))]
    recurrence_unit: String,
    #[arg(short, long)]
    recurrence: Option<i64>,
    #[arg(short = 'm', long, default_value_t = false)]
    required: bool,
    #[arg(long)]
    required_task: Option<String>,
    // #[arg(short, long, default_value_t = false)]
    // json: bool,
}
impl AddArgs {
    pub fn run(&self) {
        // println!(
        //     "{}, {}, {}, {}",
        //     self.name, self.project, self.recurrence_type, self.recurrence_unit
        // );
        let t = Task {
            id: Task::get_new_id(),
            uuid: Uuid::new_v4().to_string(),
            name: self.name.to_owned(),
            project: self.project.to_owned(),
            schedule: self.schedule.to_owned(),
            deadline: self.deadline.to_owned(),
            recurrence_type: Some(self.recurrence_type.to_owned()),
            recurrence_unit: Some(self.recurrence_unit.to_owned()),
            recurrence: self.recurrence.to_owned(),
            required: self.required,
            required_task: self.required_task.to_owned(),
            completions: Vec::new(),
            intervals: Vec::new(),
        };
        Task::append(&t);
        println!(
            "{}",
            serde_json::to_string_pretty(&t).unwrap_or(String::new())
        )
    }
}
