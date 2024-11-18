use crate::tasks::list::TaskListVecExt;
use crate::{storage::TaskStorage, tasks::query::TaskQuery};

use crate::{
    config::Config,
    tasks::{error::TaskError, list::TaskList},
};

use super::{EditParams, ModifyParams, RemoveParams, UndoneParams};

pub fn handle_remove(storage: TaskStorage, params: RemoveParams) -> Result<(), TaskError> {
    let tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(params.query)?;

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

pub fn handle_edit(config: Config, params: EditParams) -> Result<(), TaskError> {
    let editor = match std::env::var("EDITOR") {
        Ok(value) => value,
        // TODO: check if nvim -> vim -> nano is in the path, else bailout
        // TODO: add (better) loggin / log that if you want to chose the editor, set the EDITOR env
        // var
        Err(_) => "nvim".to_string(),
    };
    let mut cmd = std::process::Command::new(editor);

    if let Some(item) = params.item {
        cmd.arg(format!("+{item}"));
    }

    if let Err(e) = cmd.arg(config.todo_file()).status() {
        // TODO: use a logging library instead of `eprintln!`
        eprintln!("Failed to edit the todo file: {}", e);
        return Err(TaskError::FailedToOpenTodoFile);
    }

    Ok(())
}

pub fn handle_clean(storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;

    tasks.retain(|i| !i.task.completed);

    storage.perist(tasks)
}

pub fn handle_undone(storage: TaskStorage, params: UndoneParams) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(params.query)?;

    tasks
        .filter_mut_from_query(&query)
        .for_each(|item| item.task.undo());

    storage.perist(tasks)
}

// TODO: a query or an argument to list tasks due today, tomorrow, this week, next week, this
// month, next month
// For now we'll just list all due tasks by date
pub fn handle_due(storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;

    // TODO: is there a way to have a less leaky interface for this?
    // It'd probably not be the job of the list to know about due stuff.
    tasks.retain(|item| item.task.due_date.is_some());
    tasks.sort_by_key(|item| item.task.due_date);

    storage.perist(tasks)
}

// TODO: https://github.com/just1602/todors/issues/5
pub fn handle_modify(storage: TaskStorage, params: ModifyParams) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(params.query)?;

    tasks
        .filter_mut_from_query(&query)
        .for_each(|item| item.task.priority = params.priority);

    storage.perist(tasks)
}

pub fn print_tasks_list(tasks: TaskList, total: usize) {
    // FIXME: find the right way to display colors for completed and prioritized tasks
    // Maybe the solution is to put the logic in list item
    let width: usize = ((tasks.len() + 1).checked_ilog10().unwrap_or(0) + 1)
        .try_into()
        .expect("Failed to parse task list length width");
    for item in &tasks {
        println!("{:0width$}) {}", item.idx, item.task, width = width);
    }
    println!("⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯");
    println!("{}/{} tasks where printed", tasks.len(), total);
}
