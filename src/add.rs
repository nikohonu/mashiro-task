use uuid::Uuid;

use chrono::{Local, NaiveDateTime};

use crate::{recurrence::Recurrence, task::Task};

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    name: String,
    sphere: String,
    #[arg(short, long, default_value_t = 0)]
    order: u8,
    #[arg(short = 'd', long)]
    schedule: Option<NaiveDateTime>,
    #[arg(short, long)]
    recurrence: Option<String>,
}

impl AddArgs {
    pub fn run(&self) {
        let recurrence = match &self.recurrence {
            Some(r_string) => match Recurrence::from_string(r_string.as_str()) {
                Ok(r) => Some(r),
                Err(_) => panic!("You provide invalid recurrence."),
            },
            _ => None,
        };
        let schedule = if recurrence.is_some() && self.schedule.is_none() {
            Some(NaiveDateTime::from(Local::now().date_naive()))
        } else {
            self.schedule
        };
        let task = Task {
            uuid: Uuid::new_v4().to_string(),
            id: Task::get_new_id(),
            order: self.order,
            created: Local::now().naive_local(),
            name: self.name.to_owned(),
            sphere: self.sphere.to_owned(),
            schedule,
            recurrence,
        };
        Task::append(&task, false);
        println!("Task created!");
        println!(
            "{}",
            serde_json::to_string_pretty(&task).unwrap_or_default()
        )
    }
}
