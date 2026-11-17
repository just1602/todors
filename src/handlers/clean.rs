use crate::{cli::Clean, error::Result, storage::TaskStorage};

pub fn handle_clean(_params: Clean, storage: TaskStorage) -> Result<()> {
    let mut tasks = storage.get_all()?;

    tasks.retain(|t| !t.completed);

    storage.persist(tasks)
}
