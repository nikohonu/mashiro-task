use crate::task::Task;
use chrono::Local;

use rand::seq::SliceRandom;

use std::cmp::max;

#[derive(clap::Args, Debug)]
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
        let now = Local::now().naive_local();
        let today = now.date();
        let tasks = Task::all();
        let mut scheduled_tasks = Vec::new();
        for task in &tasks {
            if task.schedule <= now {
                scheduled_tasks.push(task.clone())
            }
        }
        let mut now_tasks = Vec::new();
        let mut relevant_tasks = Vec::new();
        for task in &scheduled_tasks {
            if let Some(now_date) = task.now_date {
                if now_date == today {
                    now_tasks.push(task.clone());
                } else {
                    relevant_tasks.push(task.clone())
                }
            } else if let Some(uuid) = &task.required_task {
                if get_task_by_uuid(&scheduled_tasks, uuid.as_str()).is_none() {
                    relevant_tasks.push(task.clone())
                }
            } else {
                relevant_tasks.push(task.clone())
            }
        }
        relevant_tasks.shuffle(&mut rand::thread_rng());
        let mut required_tasks: Vec<Task> = relevant_tasks
            .clone()
            .into_iter()
            .filter(|task| task.required)
            .collect();
        let mut optional_tasks: Vec<Task> = relevant_tasks
            .into_iter()
            .filter(|task| !task.required)
            .collect();
        let need = 3 - now_tasks.len();
        if need > 0 {
            let mut have_required = 0;
            let mut have_optional = 0;
            for task in &now_tasks {
                if task.required {
                    have_required += 1
                } else {
                    have_optional += 1
                }
            }
            let mut need_required = max(2 - have_required, 0);
            let mut need_optional = max(1 - have_optional, 0);
            while need_required != 0 && !required_tasks.is_empty() {
                if let Some(task) = required_tasks.pop() {
                    need_required -= 1;
                    now_tasks.push(task);
                }
            }
            while need_optional != 0 && !optional_tasks.is_empty() {
                if let Some(task) = optional_tasks.pop() {
                    need_optional -= 1;
                    now_tasks.push(task);
                }
            }
            while (!required_tasks.is_empty() || !optional_tasks.is_empty()) && now_tasks.len() < 3
            {
                if let Some(task) = required_tasks.pop() {
                    now_tasks.push(task);
                }
                if let Some(task) = optional_tasks.pop() {
                    now_tasks.push(task);
                }
            }
        }
        for task in &mut now_tasks {
            task.now_date = Some(today);
        }
        Task::print(&now_tasks);
        Task::update(now_tasks);
    }
}
