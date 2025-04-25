use crate::{
    cli::Modify,
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

// TODO: https://github.com/just1602/todors/issues/5
pub fn handle_modify(params: Modify, storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(&params.query)?;

    let idx_to_modify: Vec<usize> = tasks
        .filter_from_query(&query)
        .map(|task| task.id)
        .collect();

    if params.priority.is_some() {
        tasks.iter_mut().for_each(|task| {
            if idx_to_modify.contains(&task.id) {
                task.priority = params.priority
            }
        });
    }

    if params.rm_priority {
        tasks.iter_mut().for_each(|task| {
            if idx_to_modify.contains(&task.id) {
                task.priority = None
            }
        });
    }

    if params.due_date.is_some() {
        tasks.iter_mut().for_each(|task| {
            if idx_to_modify.contains(&task.id) {
                task.due_date = params.due_date
            }
        });
    }

    if params.rm_due_date {
        tasks.iter_mut().for_each(|task| {
            if idx_to_modify.contains(&task.id) {
                task.due_date = None
            }
        });
    }

    storage.perist(tasks)
}
