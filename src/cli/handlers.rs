use crate::storage::TaskStorage;

use crate::tasks::{error::TaskError, list::TaskList};

pub fn handle_clean(storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;

    tasks.retain(|i| !i.task.completed);

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
