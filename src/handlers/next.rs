use crate::{
    cli::Next,
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

use crate::utils::print_tasks_list;

pub fn handle_next(params: Next, storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let total = tasks.len();

    if let Some(query) = &params.query {
        let query = TaskQuery::from_string_vec(query)?;

        tasks = tasks.filter_from_query(&query).collect();
    }

    if let Some(task) = tasks.sort_by_urgency().first() {
        // FIXME: remove this clone
        // TODO: check if this function can take a slice instead
        print_tasks_list(&vec![task.clone()], total)?;
    }

    Ok(())
}
