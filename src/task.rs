use crate::paths::get_tasks_path;
use chrono::{NaiveDateTime, NaiveTime};
use csv::WriterBuilder;
use prettytable::format;
use prettytable::row;
use serde::{Deserialize, Serialize};

use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interval {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub uuid: String,
    pub created: NaiveDateTime,
    pub id: u64,
    pub name: String,
    pub project: String,
    pub schedule: NaiveDateTime,
    pub recurrence_type: String,
    pub recurrence_unit: String,
    pub recurrence: u64,
    pub required: bool,
    pub required_task: Option<String>,
    pub now_datetime: Option<NaiveDateTime>,
    pub times_completed: u64,
}

impl Task {
    pub fn all() -> Vec<Task> {
        let tasks_path = get_tasks_path();
        if let Ok(file) = std::fs::File::open(tasks_path) {
            let mut reader = csv::Reader::from_reader(file);
            reader
                .deserialize::<Task>()
                .map(|x| x.expect("Can't read tasks files"))
                .collect()
        } else {
            Vec::new()
        }
    }
    pub fn append(task: &Task) {
        let tasks_path = get_tasks_path();
        let is_header = !tasks_path.exists();
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(tasks_path)
            .expect("Can't open file");
        let mut wrt = WriterBuilder::new()
            .has_headers(is_header)
            .from_writer(file);
        wrt.serialize(task).expect("I can't write new record.");
        wrt.flush().expect("I cant't write in file.");
    }

    pub fn get_new_id() -> u64 {
        let tasks = Task::all();
        let mut index = 0;
        for task in tasks {
            index = std::cmp::max(task.id, index);
        }
        index + 1
    }

    pub fn rewrite(tasks: Vec<Task>) {
        let tasks_path = get_tasks_path();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(tasks_path)
            .expect("Can't open file");
        let mut wrt = WriterBuilder::new().has_headers(true).from_writer(file);
        for task in &tasks {
            wrt.serialize(task).expect("I can't write new record.");
        }
        wrt.flush().expect("I cant't write in file.");
    }

    pub fn regenerate_ids() {
        let tasks = Task::all();
        let mut new_tasks = Vec::new();
        for (id, task) in tasks.iter().enumerate() {
            let mut new_task = task.clone();
            new_task.id = id as u64 + 1;
            new_tasks.push(new_task)
        }
        Task::rewrite(new_tasks)
    }

    pub fn by_id(tasks: &Vec<Task>, id: u64) -> Option<Task> {
        for task in tasks {
            if task.id == id {
                return Some(task.clone());
            }
        }
        None
    }
    pub fn update_one(new_task: Task) {
        Task::update(vec![new_task]);
    }
    pub fn update(new_tasks: Vec<Task>) {
        let mut tasks = Task::all();
        for new_task in new_tasks {
            for (index, task) in tasks.iter().enumerate() {
                if task.id == new_task.id {
                    let _ = std::mem::replace(&mut tasks[index], new_task);
                    break;
                }
            }
        }
        Task::rewrite(tasks)
    }
    pub fn remove(id: u64) {
        let mut tasks = Task::all();
        let mut index = 0;
        for task in &tasks {
            if task.id == id {
                break;
            }
            index += 1;
        }
        tasks.remove(index);
        for task in &tasks {
            println!("{:?}", task);
        }
        Task::rewrite(tasks)
    }

    pub fn print(tasks: &Vec<Task>, compact: bool) {
        let mut table = prettytable::Table::new();
        if compact {
            table.set_titles(row!["Id", "Name", "Project", "Schedule", "Recur.", "Req.", "Comp."]);
            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        } else {
            table.set_titles(row![
                "Id",
                "Name",
                "Project",
                "Schedule",
                "Recurrence",
                "Required",
                "Times completed"
            ]);
        }
        for task in tasks {
            let schedule =
                if compact && task.schedule.time() == NaiveTime::from_hms_opt(0, 0, 0).unwrap() {
                    task.schedule.date().to_string()
                } else {
                    task.schedule.to_string()
                };
            let recurrence = format!(
                "{}{}{}",
                task.recurrence_type, task.recurrence, task.recurrence_unit
            );
            table.add_row(row![
                task.id,
                task.name,
                task.project,
                schedule,
                recurrence,
                task.required,
                task.times_completed
            ]);
        }
        table.printstd();
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
