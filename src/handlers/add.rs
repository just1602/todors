use chrono::Local;

use crate::{error::Result, storage::TaskStorage, tasks::task::TaskBuilder};

use crate::cli::Add;
use crate::utils::print_tasks_list;

pub fn handle_add(params: Add, storage: TaskStorage) -> Result<()> {
    let mut tasks = storage.get_all()?;

    let task = TaskBuilder::new(tasks.len() + 1, params.task.join(" "))
        .priority(params.pri)
        .created_at(Some(Local::now().date_naive()))
        .build()?;

    // NOTE: maybe I should just have keep the writing code inline
    // FIXME: is there a way to avoid the clone here
    tasks.push(task.clone());

    print_tasks_list(&vec![task], tasks.len())?;

    storage.persist(tasks)
}
