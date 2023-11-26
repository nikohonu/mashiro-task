use crate::task::Task;
use chrono::Local;
use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::Rng;

use std::cmp::max;

#[derive(clap::Args, Debug)]
pub struct NowArgs {
    #[arg(short, long, default_value_t = false)]
    pub full: bool,
    #[arg(short, long, default_value_t = false)]
    pub random: bool,
}

fn get_task_by_uuid(tasks: &Vec<Task>, uuid: &str) -> Option<Task> {
    for task in tasks {
        if task.uuid == uuid {
            return Some(task.clone());
        }
    }
    None
}

fn pop_random_task(tasks: &mut Vec<Task>, project_random: bool) -> Option<Task> {
    if tasks.is_empty() {
        None
    } else {
        let mut rng = rand::prelude::thread_rng();
        if project_random {
            let unique_projects: HashSet<_> =
                tasks.iter().map(|task| task.project.as_str()).collect();
            let mut unique_projects: Vec<_> = unique_projects.iter().collect();
            unique_projects.shuffle(&mut rng);
            let project = unique_projects.pop().unwrap();
            loop {
                let index = rng.gen_range(0..tasks.len());
                if &tasks[index].project.as_str() == project {
                    return Some(tasks.remove(index));
                }
            }
        } else {
            let index = rng.gen_range(0..tasks.len());
            Some(tasks.remove(index))
        }
    }
}

impl NowArgs {
    pub fn run(&self) {
        let now = Local::now().naive_local();
        let today = now.date();
        let tasks = Task::all();
        let scheduled_tasks: Vec<_> = tasks
            .iter()
            .filter(|task| task.schedule <= now)
            .cloned()
            .collect();
        let mut now_tasks = Vec::new();
        let mut relevant_tasks = Vec::new();
        for task in &scheduled_tasks {
            if let Some(now_datetime) = task.now_datetime {
                let now_date = now_datetime.date();
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
        let mut required_tasks: Vec<_> = relevant_tasks
            .clone()
            .into_iter()
            .filter(|task| task.required)
            .collect();
        let mut optional_tasks: Vec<_> = relevant_tasks
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
                if let Some(task) = pop_random_task(&mut required_tasks, false) {
                    need_required -= 1;
                    now_tasks.push(task);
                }
            }
            while need_optional != 0 && !optional_tasks.is_empty() {
                if let Some(task) = pop_random_task(&mut optional_tasks, true) {
                    need_optional -= 1;
                    now_tasks.push(task);
                }
            }
            while (!required_tasks.is_empty() || !optional_tasks.is_empty()) && now_tasks.len() < 3
            {
                if let Some(task) = pop_random_task(&mut required_tasks, false) {
                    now_tasks.push(task);
                }
                if let Some(task) = pop_random_task(&mut optional_tasks, true) {
                    now_tasks.push(task);
                }
            }
        }
        for task in &mut now_tasks {
            if let Some(now_datetime) = task.now_datetime {
                let now_date = now_datetime.date();
                if now_date != today {
                    task.now_datetime = Some(now);
                }
            } else {
                task.now_datetime = Some(now);
            }
        }
        now_tasks.sort_by_key(|t| t.now_datetime);
        Task::print(&now_tasks, !self.full);
        Task::update(now_tasks.clone());
        let task = pop_random_task(&mut now_tasks, false).unwrap();
        if self.random {
            println!(
                "A whisper from the void urges you to choose this task: {}. {} - {}",
                task.id, task.name, task.project
            )
        }
    }
}
