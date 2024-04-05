use crate::task::Task;

#[derive(clap::Args, Debug)]
pub struct ListArgs {
    #[arg(short, long, default_value_t = false)]
    pub full: bool,
}

impl ListArgs {
    pub fn run(&self) {
        let mut tasks = Task::all();
        tasks.sort_unstable_by_key(|task| task.order);
        Task::print(&tasks, !self.full);
    }
}
