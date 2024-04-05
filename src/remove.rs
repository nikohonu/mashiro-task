use crate::task::Task;
#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    id: u64,
}

impl RemoveArgs {
    pub fn run(&self) {
        let tasks = Task::all();
        let task = Task::by_id(&tasks, self.id);
        if task.is_none() {
            println!("There is no task with id = {}", self.id);
            return;
        }
        let task = task.unwrap();
        println!("{:?}", task);
        Task::remove(task.id);
        println!("Task was removed")
    }
}
