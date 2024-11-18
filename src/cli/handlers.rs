use crate::storage::TaskStorage;

use crate::tasks::{error::TaskError, list::TaskList};

pub fn handle_clean(storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;

    tasks.retain(|i| !i.task.completed);

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
