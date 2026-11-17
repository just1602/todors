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

        tasks = tasks.filter_from_query(&query).collect();
    }

    if let Some(task) = tasks.sort_by_urgency().first() {
        // FIXME: remove this clone
        // TODO: check if this function can take a slice instead
        print_tasks_list(&vec![task.clone()], total)?;
    }

    Ok(())
}
