use clap::Parser;

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait},
};

use super::print_tasks_list;

#[derive(Parser)]
#[command(
    name = "next",
    about = "Show the next task to do base on the urgency task sort we have"
)]
pub struct Next;

impl Next {
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let tasks = storage.get_all()?;
        let total = tasks.len();

        if let Some(task) = tasks.sort_by_urgency().first() {
            // FIXME: remove this clone
            // TODO: check if this function can take a slice instead
            print_tasks_list(vec![task.clone()], total);
        }

        Ok(())
    }
}
