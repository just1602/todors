use crate::{
    cli::Next,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::{list::TaskListTrait, query::TaskQuery},
    utils::print_tasks_list,
};

pub fn handle_next(params: Next, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;
    let total = tasks.len();

    if let Some(query) = &params.query {
        let query = TaskQuery::from_string_vec(query).context_to::<AppError>()?;

        tasks.retain(|task| query.matches(task));
    }

    tasks.sort_by_urgency();

    print_tasks_list(tasks.first(), total)?;

    Ok(())
}
