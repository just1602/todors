use crate::{
    storage::TaskStorage,
    tasks::{list::TaskListItem, query::TaskQuery},
};

use crate::{
    config::Config,
    tasks::{error::TaskError, list::TaskList},
};

use super::{DoneParams, EditParams, ListParams, ModifyParams, RemoveParams, UndoneParams};

pub fn handle_list(storage: impl TaskStorage, params: ListParams) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let total = tasks.len();

    if !params.all {
        tasks.retain(|item| !item.task.completed)
    }

    if let Some(query) = params.query {
        let query = TaskQuery::from_string_vec(query)?;

        tasks = filter_task_from_query(&tasks, &query).collect();
    }

    print_tasks_list(tasks, total);
    Ok(())
}

pub fn handle_done(storage: impl TaskStorage, params: DoneParams) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(params.query)?;

    filter_mut_task_from_query(&mut tasks, &query).for_each(|item| item.task.complete());

    storage.perist(tasks)
}

pub fn handle_remove(storage: impl TaskStorage, params: RemoveParams) -> Result<(), TaskError> {
    let tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(params.query)?;

    let idx_to_remove: Vec<usize> = filter_task_from_query(&tasks, &query)
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

pub fn handle_clean(storage: impl TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;

    tasks.retain(|i| !i.task.completed);

    storage.perist(tasks)
}

pub fn handle_undone(storage: impl TaskStorage, params: UndoneParams) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(params.query)?;

    filter_mut_task_from_query(&mut tasks, &query).for_each(|item| item.task.undo());

    storage.perist(tasks)
}

// TODO: a query or an argument to list tasks due today, tomorrow, this week, next week, this
// month, next month
// For now we'll just list all due tasks by date
pub fn handle_due(storage: impl TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;

    // TODO: is there a way to have a less leaky interface for this?
    // It'd probably not be the job of the list to know about due stuff.
    tasks.retain(|item| item.task.due_date.is_some());
    tasks.sort_by_key(|item| item.task.due_date);

    storage.perist(tasks)
}

// TODO: https://github.com/just1602/todors/issues/5
pub fn handle_modify(storage: impl TaskStorage, params: ModifyParams) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(params.query)?;

    filter_mut_task_from_query(&mut tasks, &query)
        .for_each(|item| item.task.priority = params.priority);

    storage.perist(tasks)
}

// FIXME: is there a way to do this without cloning the whole thing?
fn filter_task_from_query<'a>(
    tasks: &'a TaskList,
    query: &'a TaskQuery,
) -> impl Iterator<Item = TaskListItem> + use<'a> {
    tasks
        .iter()
        .filter(|item| {
            if query.indexes.contains(&item.idx) {
                return true;
            }

            if item
                .task
                .projects
                .iter()
                .any(|pro| query.projects.contains(pro))
            {
                return true;
            }

            if item
                .task
                .contexts
                .iter()
                .any(|ctx| query.contexts.contains(ctx))
            {
                return true;
            }

            if let Some(due_date) = query.due_date {
                if item.task.due_date.is_some_and(|dd| dd == due_date) {
                    return true;
                }
            }

            if !query.subject.is_empty() && item.task.subject.contains(&query.subject) {
                return true;
            }

            false
        })
        .cloned()
}

fn filter_mut_task_from_query<'a>(
    tasks: &'a mut TaskList,
    query: &'a TaskQuery,
) -> impl Iterator<Item = &'a mut TaskListItem> {
    tasks.iter_mut().filter(|item| {
        if query.indexes.contains(&item.idx) {
            return true;
        }

        if item
            .task
            .projects
            .iter()
            .any(|pro| query.projects.contains(pro))
        {
            return true;
        }

        if item
            .task
            .contexts
            .iter()
            .any(|ctx| query.contexts.contains(ctx))
        {
            return true;
        }

        if let Some(due_date) = query.due_date {
            if item.task.due_date.is_some_and(|dd| dd == due_date) {
                return true;
            }
        }

        if !query.subject.is_empty() && item.task.subject.contains(&query.subject) {
            return true;
        }

        false
    })
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
