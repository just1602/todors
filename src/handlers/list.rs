use crate::{
    cli::List,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::{list::TaskListTrait, query::TaskQuery},
    utils::print_tasks_list,
};

pub fn handle_list(params: List, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;
    let total = tasks.len();

    if !params.all {
        tasks.retain(|task| !task.completed)
    }

    if let Some(query) = &params.query {
        let query = TaskQuery::from_string_vec(query).context_to::<AppError>()?;

        tasks = tasks.filter_from_query(&query).collect();
    }

    tasks.sort_by_urgency();
    print_tasks_list(&tasks, total)?;
    Ok(())
}
