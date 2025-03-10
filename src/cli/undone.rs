use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

use super::Undone;

pub fn handle_undone(params: Undone, storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(&params.query)?;

    tasks
        .filter_mut_from_query(&query)
        .for_each(|item| item.task.undo());

    storage.perist(tasks)
}
