use clap::Parser;

use crate::{storage::TaskStorage, tasks::error::TaskError};

#[derive(Parser)]
#[command(name = "clean", about = "Clean all the completed tasks")]
pub struct Clean;

impl Clean {
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;

        tasks.retain(|i| !i.task.completed);

        storage.perist(tasks)
    }
}
