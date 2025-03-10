use crate::{
    cli::Remove,
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

pub fn handle_remove(params: Remove, storage: TaskStorage) -> Result<(), TaskError> {
    let tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(&params.query)?;

    let idx_to_remove: Vec<usize> = tasks
        .filter_from_query(&query)
        .map(|item| item.idx)
        .collect();

    let tasks = tasks
        .into_iter()
        .filter(|item| !idx_to_remove.contains(&item.idx))
        .collect();

    storage.perist(tasks)
}
