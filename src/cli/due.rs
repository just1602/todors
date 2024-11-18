use clap::Parser;

use crate::{storage::TaskStorage, tasks::error::TaskError};

use super::print_tasks_list;

#[derive(Parser)]
#[command(name = "due", about = "List all due tasks")]
pub struct Due;

impl Due {
    // TODO: a query or an argument to list tasks due today, tomorrow, this week, next week, this
    // month, next month
    // For now we'll just list all due tasks by date
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;
        let total = tasks.len();

        // TODO: is there a way to have a less leaky interface for this?
        // It'd probably not be the job of the list to know about due stuff.
        tasks.retain(|item| item.task.due_date.is_some());
        tasks.sort_by_key(|item| item.task.due_date);

        print_tasks_list(tasks, total);

        Ok(())
    }
}
