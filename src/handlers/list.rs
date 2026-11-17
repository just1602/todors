use crate::{
    cli::List,
    error::Result,
    storage::TaskStorage,
    tasks::{list::TaskListTrait, query::TaskQuery},
};

use crate::utils::print_tasks_list;

pub fn handle_list(params: List, storage: TaskStorage) -> Result<()> {
    let mut tasks = storage.get_all()?;
    let total = tasks.len();

    if !params.all {
        tasks.retain(|task| !task.completed)
    }

    if let Some(query) = &params.query {
        let query = TaskQuery::from_string_vec(query)?;

        tasks = tasks.filter_from_query(&query).collect();
    }

    tasks = tasks.sort_by_urgency();
    print_tasks_list(&tasks, total)?;
    Ok(())
}
