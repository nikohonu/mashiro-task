use crate::task::Task;
use chrono::{Local, NaiveDateTime};

use rand::Rng;

#[derive(clap::Args, Debug)]
pub struct NowArgs {
    #[arg(short, long, default_value_t = false)]
    pub full: bool,
    #[arg(short, long, default_value_t = false)]
    pub random: bool,
}

fn pop_random_task(tasks: &mut Vec<Task>) -> Option<Task> {
    if tasks.is_empty() {
        None
    } else {
        let mut rng = rand::prelude::thread_rng();
        let index = rng.gen_range(0..tasks.len());
        Some(tasks.remove(index))
    }
}

fn task_filter(task: &&Task, now: NaiveDateTime) -> bool {
    if let Some(schedule) = task.schedule {
        schedule <= now
    } else {
        true
    }
}

impl NowArgs {
    pub fn run(&self) {
        let now = Local::now().naive_local();
        let tasks = Task::all();
        let mut scheduled_tasks: Vec<_> = tasks
            .iter()
            .filter(|task| task_filter(task, now))
            .cloned()
            .collect();
        scheduled_tasks.sort_unstable_by_key(|task| task.order);
        Task::print(&scheduled_tasks, !self.full);
        if self.random {
            if let Some(task) = pop_random_task(&mut scheduled_tasks) {
                println!(
                    "A whisper from the void urges you to choose this task: {}. {} - {}",
                    task.id, task.name, task.sphere
                )
            }
        }
    }
}
