use chrono::Local;

use crate::tasks::{list::TaskListItem, query::TaskQuery};
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::{
    config::Config,
    tasks::{error::TaskError, list::TaskList, task::Task},
};

use super::{AddParams, DoneParams, ListParams, RemoveParams};

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
    // let mut tasks = read_tasks_from_file(&config)?;
    //
    // let query = params.query.join(" ");

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
