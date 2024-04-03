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
        let tasks = Task::all();
        let mut scheduled_tasks: Vec<_> = tasks
            .iter()
            .filter(|task| task.schedule <= now)
            .cloned()
            .collect();
        Task::print(&scheduled_tasks, !self.full);
        Task::update(scheduled_tasks.clone());
        match pop_random_task(&mut scheduled_tasks, false) {
            Some(task) => {
                if self.random {
                    println!(
                        "A whisper from the void urges you to choose this task: {}. {} - {}",
                        task.id, task.name, task.project
                    )
                }
            }
            None => {}
        }
    }
}
