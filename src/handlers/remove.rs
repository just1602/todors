use crate::{
    cli::Remove,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::query::TaskQuery,
};

pub fn handle_remove(params: Remove, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;
    let query = TaskQuery::from_string_vec(&params.query).context_to::<AppError>()?;

    tasks.retain(|task| !query.matches(task));

    storage.persist(tasks).context_to::<AppError>()
}
