use crate::{
    cli::Modify,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::query::TaskQuery,
};

// TODO: https://github.com/just1602/todors/issues/5
pub fn handle_modify(params: Modify, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;
    let query = TaskQuery::from_string_vec(&params.query).context_to::<AppError>()?;

    for task in tasks.iter_mut().filter(|task| query.matches(task)) {
        if let Some(priority) = params.priority {
            task.priority = Some(priority);
        }

        if params.rm_priority {
            task.priority = None;
        }

        if let Some(due_date) = params.due_date {
            task.due_date = Some(due_date);
        }

        if params.rm_due_date {
            task.due_date = None;
        }
    }

    storage.persist(tasks).context_to::<AppError>()
}
