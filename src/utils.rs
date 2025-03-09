use crate::tasks::error::TaskError;
use crate::tasks::list::{TaskList, TaskListTrait};
use colored::Colorize;
use std::io;
use std::io::Write;

pub fn print_tasks_list(tasks: &TaskList, total: usize) -> Result<(), TaskError> {
    let tasks = tasks.sort_by_urgency();

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    // FIXME: find the right way to display colors for completed and prioritized tasks
    // Maybe the solution is to put the logic in list item
    let width: usize = ((tasks.len() + 1).checked_ilog10().unwrap_or(0) + 1)
        .try_into()
        .expect("Failed to parse task list length width");
    for item in &tasks {
        let mut line = format!("{:0width$}) {}", item.idx, item.task, width = width);
        if let Some(priority) = item.task.priority {
            line = match priority {
                'A' => line.magenta().bold().to_string(),
                'B' => line.yellow().bold().to_string(),
                'C' => line.green().bold().to_string(),
                _ => line.blue().bold().to_string(),
            };
        }
        match writeln!(handle, "{}", line) {
            Ok(_) => {}
            Err(err) => {
                eprint!("Failed to write tasks list to stdout: {}", err);
                return Err(TaskError::FailedToWriteToStdout);
            }
        }
    }
    match writeln!(
        handle,
        "⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯\n{}/{} tasks where printed",
        tasks.len(),
        total
    ) {
        Ok(_) => {}
        Err(err) => {
            eprint!("Failed to write tasks list to stdout: {}", err);
            return Err(TaskError::FailedToWriteToStdout);
        }
    }

    Ok(())
}
