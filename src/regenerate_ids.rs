use crate::task::Task;
#[derive(clap::Args, Debug)]
pub struct RegenerateIdsArgs {}

impl RegenerateIdsArgs {
    pub fn run(&self) {
        Task::regenerate_ids()
    }
}
