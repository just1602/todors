use chrono::Local;

use crate::{
    cli::Add,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::task::TaskBuilder,
};

use crate::utils::print_tasks_list;

pub fn handle_add(params: Add, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;

    let task = TaskBuilder::new(tasks.len() + 1, params.task.join(" "))
        .priority(params.pri)
        .created_at(Some(Local::now().date_naive()))
        .build()
        .context_to::<AppError>()?;

    // NOTE: maybe I should just have keep the writing code inline
    // FIXME: is there a way to avoid the clone here
    tasks.push(task.clone());

    print_tasks_list(&vec![task], tasks.len())?;

    storage.persist(tasks).context_to::<AppError>()
}
