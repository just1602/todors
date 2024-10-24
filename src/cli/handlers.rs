use chrono::Local;

use crate::tasks::{list::TaskListItem, query::TaskQuery};
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::{
    config::Config,
    tasks::{error::TaskError, list::TaskList, task::Task},
};

use super::{AddParams, DoneParams, EditParams, ListParams, RemoveParams};

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

    if !params.all {
        tasks.retain(|item| !item.task.completed)
    }

    if let Some(query) = params.query {
        // the query in the `ListParams` struct must be a `Vec<String>` to avoid the need of
        // quoting, so we join it before parsing it
        let query = query.join(" ");

        if let Ok(task_query) = query.parse::<TaskQuery>() {
            if !task_query.indexes.is_empty() {
                tasks.retain(|item| task_query.indexes.contains(&item.idx));
                print_tasks_list(tasks);
                // returns early since we don't want to handle anything else when we have an index
                // or a range
                return Ok(());
            }

            if !task_query.projects.is_empty() {
                tasks.retain(|item| {
                    item.task
                        .projects
                        .iter()
                        .any(|pro| task_query.projects.contains(pro))
                });
            }

            if !task_query.contexts.is_empty() {
                tasks.retain(|item| {
                    item.task
                        .contexts
                        .iter()
                        .any(|ctx| task_query.contexts.contains(ctx))
                });
            }

            if let Some(due_date) = task_query.due_date {
                tasks.retain(|item| item.task.due_date.is_some_and(|dd| dd == due_date))
            }
        }
    }

    print_tasks_list(tasks);
    Ok(())
}

pub fn handle_done(config: Config, params: DoneParams) -> Result<(), TaskError> {
    let mut tasks = read_tasks_from_file(&config)?;

    let query = params.query.join(" ");

    if let Ok(query) = query.parse::<TaskQuery>() {
        // TODO: display the task that are marked as done
        if !query.indexes.is_empty() {
            tasks.iter_mut().for_each(|item| {
                if query.indexes.contains(&item.idx) {
                    item.task.complete()
                }
            });

            return persist_tasks(config.todo_file(), tasks);
        }

        // We should probably extract those in a function and just pass the lambda to it if
        // possible
        if !query.projects.is_empty() {
            tasks.iter_mut().for_each(|item| {
                if item
                    .task
                    .projects
                    .iter()
                    .any(|pro| query.projects.contains(pro))
                {
                    item.task.complete()
                }
            })
        }

        if !query.contexts.is_empty() {
            tasks.iter_mut().for_each(|item| {
                if item
                    .task
                    .contexts
                    .iter()
                    .any(|ctx| query.contexts.contains(ctx))
                {
                    item.task.complete()
                }
            })
        }

        if let Some(due_date) = query.due_date {
            tasks.iter_mut().for_each(|item| {
                if item.task.due_date.is_some_and(|dd| dd == due_date) {
                    item.task.complete();
                }
            });
        }

        persist_tasks(config.todo_file(), tasks)
    } else {
        Err(TaskError::FailedToParseQuery)
    }
}

pub fn handle_remove(config: Config, params: RemoveParams) -> Result<(), TaskError> {
    let tasks = read_tasks_from_file(&config)?;

    let query = params.query.join(" ");

    if let Ok(query) = query.parse::<TaskQuery>() {
        if !query.indexes.is_empty() {
            let remaning_tasks = tasks
                .into_iter()
                .filter(|item| !query.indexes.contains(&item.idx))
                .collect();

            return persist_tasks(config.todo_file(), remaning_tasks);
        }

        let mut remaning_tasks = TaskList::new();

        if !query.projects.is_empty() {
            remaning_tasks = tasks
                .into_iter()
                .filter(|item| {
                    !item
                        .task
                        .projects
                        .iter()
                        .any(|pro| query.projects.contains(pro))
                })
                .collect();
        }

        if !query.contexts.is_empty() {
            remaning_tasks = remaning_tasks
                .into_iter()
                .filter(|item| {
                    !item
                        .task
                        .contexts
                        .iter()
                        .any(|ctx| query.contexts.contains(ctx))
                })
                .collect();
        }

        if let Some(due_date) = query.due_date {
            remaning_tasks = remaning_tasks
                .into_iter()
                .filter(|item| !item.task.due_date.is_some_and(|dd| dd == due_date))
                .collect();
        }

        persist_tasks(config.todo_file(), remaning_tasks)
    } else {
        Err(TaskError::FailedToParseQuery)
    }
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

fn print_tasks_list(tasks: TaskList) {
    // FIXME: find the right way to display colors for completed and prioritized tasks
    // Maybe the solution is to put the logic in list item
    let width: usize = ((tasks.len() + 1).checked_ilog10().unwrap_or(0) + 1)
        .try_into()
        .expect("Failed to parse task list length width");
    for item in tasks {
        println!("{:0width$}) {}", item.idx, item.task, width = width);
    }
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
