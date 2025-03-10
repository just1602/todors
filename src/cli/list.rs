use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

use crate::utils::print_tasks_list;

use super::List;

pub fn handle_list(params: List, storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let total = tasks.len();

    if !params.all {
        tasks.retain(|item| !item.task.completed)
    }

    if let Some(query) = &params.query {
        let query = TaskQuery::from_string_vec(query)?;

        tasks = tasks.filter_from_query(&query).collect();
    }

    print_tasks_list(&tasks, total)?;
    Ok(())
}
