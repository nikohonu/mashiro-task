use std::fs::OpenOptions;

use chrono::{NaiveDateTime, NaiveTime};
use csv::WriterBuilder;
use prettytable::{format, row};
use serde::{Deserialize, Serialize};

use crate::paths::{get_archive_path, get_tasks_path};
use crate::recurrence::Recurrence;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub uuid: String,
    pub id: u64,
    pub order: u8,
    pub name: String,
    pub sphere: String,
    pub schedule: Option<NaiveDateTime>,
    pub recurrence: Option<Recurrence>,
    pub created: NaiveDateTime,
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

    pub fn append(task: &Task, archive: bool) {
        let tasks_path = if archive {
            get_archive_path()
        } else {
            get_tasks_path()
        };
        let is_header = if tasks_path.exists() && tasks_path.metadata().unwrap().len() == 0 {
            true
        } else {
            !tasks_path.exists()
        };
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
    // pub fn by_uuid(tasks: &Vec<Task>, uuid: &str) -> Option<Task> {
    //     for task in tasks {
    //         if task.uuid == uuid {
    //             return Some(task.clone());
    //         }
    //     }
    //     None
    // }

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
        Task::rewrite(tasks)
    }

    pub fn print(tasks: &Vec<Task>, compact: bool) {
        let mut table = prettytable::Table::new();
        if compact {
            table.set_titles(row![
                "Id", "Ord.", "Name", "Sphere", "Sched.", "Recur.", "Dur."
            ]);
            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        } else {
            table.set_titles(row![
                "Id",
                "Order",
                "Name",
                "Sphere",
                "Schedule",
                "Recurrence",
                "Duration"
            ]);
        }
        for task in tasks {
            table.add_row(row![
                task.id,
                task.order,
                task.name,
                task.sphere,
                if let Some(schedule) = task.schedule {
                    if compact && schedule.time() == NaiveTime::from_hms_opt(0, 0, 0).unwrap() {
                        schedule.date().to_string()
                    } else {
                        schedule.to_string()
                    }
                } else {
                    String::new()
                },
                if let Some(recurrence) = &task.recurrence {
                    recurrence.to_string()
                } else {
                    String::new()
                }
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
