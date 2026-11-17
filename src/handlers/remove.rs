use crate::{
    cli::Remove,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::{list::TaskListTrait, query::TaskQuery},
};

pub fn handle_remove(params: Remove, storage: TaskStorage) -> AppResult<()> {
    let tasks = storage.get_all().context_to::<AppError>()?;
    let query = TaskQuery::from_string_vec(&params.query).context_to::<AppError>()?;

    let idx_to_remove: Vec<usize> = tasks
        .filter_from_query(&query)
        .map(|task| task.id)
        .collect();

    let tasks = tasks
        .into_iter()
        .filter(|task| !idx_to_remove.contains(&task.id))
        .collect();

    storage.persist(tasks).context_to::<AppError>()
}
