use crate::{cli::Clean, storage::TaskStorage, tasks::error::TaskError};

pub fn handle_clean(_params: Clean, storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;

    tasks.retain(|i| !i.task.completed);

    storage.perist(tasks)
}
