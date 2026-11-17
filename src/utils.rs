use crate::tasks::error::TaskError;
use crate::tasks::list::TaskList;
use colored::Colorize;
use std::io;
use std::io::Write;

pub fn print_tasks_list(tasks: &TaskList, total: usize) -> Result<(), TaskError> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    // FIXME: find the right way to display colors for completed and prioritized tasks
    // Maybe the solution is to put the logic in list item
    let width = usize::try_from((tasks.len() + 1).checked_ilog10().unwrap_or(0) + 1).unwrap_or(1);
    for task in tasks {
        let mut line = format!("{:0width$}) {}", task.id, task, width = width);
        if let Some(priority) = task.priority {
            line = match priority {
                'A' => line.magenta().bold().to_string(),
                'B' => line.yellow().bold().to_string(),
                'C' => line.green().bold().to_string(),
                _ => line.blue().bold().to_string(),
            };
        }
        if let Err(err) = writeln!(handle, "{line}") {
            eprint!("Failed to write tasks list to stdout: {err}");
            return Err(TaskError::FailedToWriteToStdout);
        }
    }
    if let Err(err) = writeln!(
        handle,
        "⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯\n{}/{} tasks where printed",
        tasks.len(),
        total
    ) {
        eprint!("Failed to write tasks list to stdout: {err}");
        return Err(TaskError::FailedToWriteToStdout);
    }

    if let Err(err) = handle.flush() {
        eprint!("Failed to write tasks list to stdout: {err}");
        return Err(TaskError::FailedToWriteToStdout);
    }

    Ok(())
}
