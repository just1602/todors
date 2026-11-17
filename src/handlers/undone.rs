use crate::{
    cli::Undone,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::{list::TaskListTrait, query::TaskQuery},
};

pub fn handle_undone(params: Undone, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;
    let query = TaskQuery::from_string_vec(&params.query).context_to::<AppError>()?;

    tasks
        .filter_mut_from_query(&query)
        .for_each(|task| task.undo());

    storage.persist(tasks).context_to::<AppError>()
}
