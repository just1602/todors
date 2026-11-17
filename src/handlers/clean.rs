use crate::{
    cli::Clean,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
};

pub fn handle_clean(_params: Clean, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;

    tasks.retain(|t| !t.completed);

    storage.persist(tasks).context_to::<AppError>()
}
