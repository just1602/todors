use chrono::Local;

use crate::tasks::{list::TaskListItem, query::TaskQuery};
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::{
    config::Config,
    tasks::{error::TaskError, list::TaskList, task::Task},
};

use super::{
    AddParams, DoneParams, EditParams, ListParams, ModifyParams, RemoveParams, UndoneParams,
};

pub fn handle_add(config: Config, params: AddParams) -> Result<(), TaskError> {
    let task = params.task.join(" ");
    let mut task: Task = task.parse()?;
    // FIXME: this should probably be in a factory method
    task.created_at = Some(Local::now().date_naive());

    // FIXME: this should probably be in a factory method
    if params.pri.is_some() && task.priority.is_none() {
        task.priority = params.pri
    }

    // FIXME: this could be seen as an "optimisation",
    // but really, we should fetch the list,
    // add the new task to it, and persist the list again
    let mut file = if let Ok(file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(config.todo_file())
    {
        file
    } else {
        return Err(TaskError::FailedToOpenTodoFile);
    };

    match file.write_fmt(format_args!("{}\n", task)) {
        Ok(_) => Ok(()),
        // FIXME: find a way to add the error as source
        Err(_) => Err(TaskError::FailedToSave),
    }
}

pub fn handle_list(config: Config, params: ListParams) -> Result<(), TaskError> {
    let mut tasks = read_tasks_from_file(&config)?;
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

pub fn handle_done(config: Config, params: DoneParams) -> Result<(), TaskError> {
    let mut tasks = read_tasks_from_file(&config)?;
    let query = TaskQuery::from_string_vec(params.query)?;

    filter_mut_task_from_query(&mut tasks, &query).for_each(|item| item.task.complete());

    persist_tasks(config.todo_file(), tasks)
}

pub fn handle_remove(config: Config, params: RemoveParams) -> Result<(), TaskError> {
    let tasks = read_tasks_from_file(&config)?;
    let query = TaskQuery::from_string_vec(params.query)?;

    let idx_to_remove: Vec<usize> = filter_task_from_query(&tasks, &query)
        .map(|item| item.idx)
        .collect();

    let tasks = tasks
        .into_iter()
        .filter(|item| !idx_to_remove.contains(&item.idx))
        .collect();

    persist_tasks(config.todo_file(), tasks)
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

pub fn handle_clean(config: Config) -> Result<(), TaskError> {
    let mut tasks = read_tasks_from_file(&config)?;

    tasks.retain(|i| !i.task.completed);

    persist_tasks(config.todo_file(), tasks)
}

pub fn handle_undone(config: Config, params: UndoneParams) -> Result<(), TaskError> {
    let mut tasks = read_tasks_from_file(&config)?;
    let query = TaskQuery::from_string_vec(params.query)?;

    filter_mut_task_from_query(&mut tasks, &query).for_each(|item| item.task.undo());

    persist_tasks(config.todo_file(), tasks)
}

// TODO: a query or an argument to list tasks due today, tomorrow, this week, next week, this
// month, next month
// For now we'll just list all due tasks by date
pub fn handle_due(config: Config) -> Result<(), TaskError> {
    let mut tasks = read_tasks_from_file(&config)?;
    let total = tasks.len();

    // TODO: is there a way to have a less leaky interface for this?
    // It'd probably not be the job of the list to know about due stuff.
    tasks.retain(|item| item.task.due_date.is_some());
    tasks.sort_by_key(|item| item.task.due_date);

    print_tasks_list(tasks, total);

    Ok(())
}

// TODO: https://github.com/just1602/todors/issues/5
pub fn handle_modify(config: Config, params: ModifyParams) -> Result<(), TaskError> {
    let mut tasks = read_tasks_from_file(&config)?;
    let query = TaskQuery::from_string_vec(params.query)?;

    filter_mut_task_from_query(&mut tasks, &query)
        .for_each(|item| item.task.priority = params.priority);

    persist_tasks(config.todo_file(), tasks)
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

fn print_tasks_list(tasks: TaskList, total: usize) {
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

fn read_tasks_from_file(config: &Config) -> Result<TaskList, TaskError> {
    let Ok(content) = std::fs::read_to_string(config.todo_file()) else {
        return Err(TaskError::FailedToOpenTodoFile);
    };

    let mut tasks = TaskList::new();
    for (idx, line) in content.lines().enumerate() {
        let Ok(task) = line.parse::<Task>() else {
            return Err(TaskError::FailedToParse);
        };

        tasks.push(TaskListItem { idx: idx + 1, task })
    }

    Ok(tasks)
}

// TODO: move this function in it's own module
// TODO: create a storage struct that would contain the dir path / file path instead of passing
// config around
fn persist_tasks(file: PathBuf, tasks: TaskList) -> Result<(), TaskError> {
    let mut file = if let Ok(file) = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file)
    {
        file
    } else {
        return Err(TaskError::FailedToOpenTodoFile);
    };

    for item in tasks {
        match file.write_fmt(format_args!("{}\n", item.task)) {
            Ok(_) => {}
            Err(_) => return Err(TaskError::FailedToSave),
        }
    }

    Ok(())
}
