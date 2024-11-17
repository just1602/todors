use clap::{command, Parser};

use crate::{
    cli::handlers::print_tasks_list,
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListItem, task::TaskBuilder},
};

#[derive(Parser)]
#[command(name = "add", visible_alias = "a", about = "Add a task to the list")]
pub struct Add {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    task: Vec<String>,

    #[arg(long, help = "Set the priority directly after creating the task")]
    pri: Option<char>,
}

impl Add {
    pub fn new(task: Vec<String>, pri: Option<char>) -> Self {
        Add { task, pri }
    }

    pub fn execute(self, storage: TaskStorage) -> Result<(), TaskError> {
        let task = TaskBuilder::new(self.task.join(" "))
            .priority(self.pri)
            .build()?;

        let mut tasks = storage.get_all()?;

        let item = TaskListItem {
            idx: tasks.len() + 1,
            task: task.clone(),
        };

        // NOTE: maybe I should just have keep the writing code inline
        // FIXME: implement copy for `Task` and `TaskListItem`
        tasks.push(item.clone());

        print_tasks_list(vec![item], tasks.len());

        storage.perist(tasks)
    }
}
