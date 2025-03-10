use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListItem, task::TaskBuilder},
};

use crate::cli::Add;
use crate::utils::print_tasks_list;

pub fn handle_add(params: Add, storage: TaskStorage) -> Result<(), TaskError> {
    let task = TaskBuilder::new(params.task.join(" "))
        .priority(params.pri)
        .build()?;

    let mut tasks = storage.get_all()?;

    let item = TaskListItem {
        idx: tasks.len() + 1,
        task: task.clone(),
    };

    // NOTE: maybe I should just have keep the writing code inline
    // FIXME: implement copy for `Task` and `TaskListItem`
    tasks.push(item.clone());

    print_tasks_list(&vec![item], tasks.len())?;

    storage.perist(tasks)
}
