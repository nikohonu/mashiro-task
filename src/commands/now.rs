use anyhow::Result;

use crate::{task::Task, tasks::Tasks};

#[derive(clap::Args, Debug)]
pub struct Now {
    max_tasks: Option<usize>,
}

impl Now {
    pub fn filter_task(task: &Task) -> bool {
        task.schedule
            .as_ref()
            .map_or(false, |schedule| schedule.is_ready())
    }

    pub fn run(&self) -> Result<()> {
        let mut tasks = Tasks::new()?;
        tasks.filter(Self::filter_task);
        tasks.sort(true);
        tasks.reduce(self.max_tasks.unwrap_or(2));
        tasks.print();
        Ok(())
    }
}
