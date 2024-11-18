use crate::tasks::list::TaskList;

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
