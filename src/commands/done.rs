use anyhow::Result;

use crate::tasks::Tasks;
#[derive(clap::Args, Debug)]
pub struct Done {
    id: String,
}
impl Done {
    pub fn run(&self) -> Result<()> {
        let mut tasks = Tasks::new()?;
        tasks.done(&self.id);
        Ok(())
    }
}
