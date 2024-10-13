use anyhow::Result;

use crate::{task::Task, tasks::Tasks};

#[derive(clap::Args, Debug)]
pub struct Other {
    #[arg(short, long, default_value_t = 3)]
    max_tasks: usize,
}

impl Other {
    pub fn filter_task(task: &Task) -> bool {
        task.schedule.is_none()
    }

    pub fn run(&self) -> Result<()> {
        let mut tasks = Tasks::new()?;
        tasks.filter(Self::filter_task);
        tasks.print();
        Ok(())
    }
}
