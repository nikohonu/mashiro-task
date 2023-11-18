use crate::{paths, task::Task};
use chrono::{Local, NaiveDate};

use prettytable::row;
use rand::{seq::SliceRandom};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{OpenOptions},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NowState {
    date: NaiveDate,
    tasks: Vec<String>,
}

fn save_state(now_state: NowState) {
    let now_state_path = paths::get_now_state_path();
    let writer = OpenOptions::new()
        .create(true)
        .write(true)
        .open(now_state_path)
        .expect("Can't open file");
    serde_json::to_writer_pretty(writer, &now_state).expect("Can't deserialize")
}
fn load_state() -> NowState {
    let now_state_path = paths::get_now_state_path();
    match OpenOptions::new().read(true).open(now_state_path) {
        Ok(r) => serde_json::from_reader(r).expect("Can't parce NowState"),
        _ => NowState {
            date: Local::now().naive_local().date(),
            tasks: Vec::new(),
        },
    }
}

#[derive(clap::Args)]
pub struct NowArgs {}

fn get_task_by_uuid(tasks: &Vec<Task>, uuid: &str) -> Option<Task> {
    for task in tasks {
        if task.uuid == uuid {
            return Some(task.clone());
        }
    }
    None
}

impl NowArgs {
    pub fn run(&self) {
        println!("Now");
        let now = Local::now().naive_local();
        let now_state = load_state();
        let mut all_tasks = Task::get_tasks();
        let mut selected_tasks: Vec<Task> = vec![];
        if now_state.date == now.date() && !now_state.tasks.is_empty() {
            for uuid in now_state.tasks {
                selected_tasks
                    .push(get_task_by_uuid(&all_tasks, uuid.as_str()).expect("Can't add task"))
            }
        } else {
            println!("{:?}", now_state);
            let mut rng = rand::thread_rng();
            all_tasks.shuffle(&mut rng);
            let mut relevant_tasks: Vec<Task> = vec![];
            for task in &all_tasks {
                if let Some(schedule) = task.schedule {
                    if schedule < now {
                        relevant_tasks.push(task.clone())
                    }
                } else {
                    relevant_tasks.push(task.clone())
                }
            }
            let mut tasks: Vec<Task> = vec![];
            for task in &relevant_tasks {
                if let Some(uuid) = &task.required_task {
                    if get_task_by_uuid(&relevant_tasks, uuid.as_str()).is_none() {
                        tasks.push(task.clone())
                    }
                } else {
                    tasks.push(task.clone())
                }
            }
            let mut required_tasks: Vec<Task> = tasks
                .clone()
                .into_iter()
                .filter(|task| task.required)
                .collect();
            let mut optional_tasks: Vec<Task> =
                tasks.into_iter().filter(|task| !task.required).collect();
            while selected_tasks.len() < 3 {
                if let Some(task) = optional_tasks.pop() {
                    selected_tasks.push(task)
                } else if let Some(task) = required_tasks.pop() {
                    selected_tasks.push(task)
                } else {
                    break;
                }
            }
            save_state(NowState {
                date: now.date(),
                tasks: selected_tasks.clone().into_iter().map(|x| x.uuid).collect(),
            });
        }
        let mut table = prettytable::Table::new();
        table.set_titles(row!["UUID", "Name", "Project", "Schedule", "Recurrence"]);
        for task in &selected_tasks {
            let schedule = if let Some(s) = task.schedule {
                s.to_string()
            } else {
                String::new()
            };
            let recurrence = if let (Some(r), Some(u)) = (task.recurrence, &task.recurrence_unit) {
                format!("{}{}", r, u)
            } else {
                String::new()
            };
            table.add_row(row![
                task.uuid,
                task.name,
                task.project,
                schedule,
                recurrence
            ]);
        }
        table.printstd();
    }
}
