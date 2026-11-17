use crate::cli::Due;
use crate::error::Result;
use crate::storage::TaskStorage;

use crate::utils::print_tasks_list;

// TODO: a query or an argument to list tasks due today, tomorrow, this week, next week, this
// month, next month
// For now we'll just list all due tasks by date
pub fn handle_due(_params: Due, storage: TaskStorage) -> Result<()> {
    let mut tasks = storage.get_all()?;
    let total = tasks.len();

    // TODO: is there a way to have a less leaky interface for this?
    // It'd probably not be the job of the list to know about due stuff.
    tasks.retain(|task| !task.completed && task.due_date.is_some());
    tasks.sort_by_key(|task| task.due_date);

    print_tasks_list(&tasks, total)?;

    Ok(())
}
