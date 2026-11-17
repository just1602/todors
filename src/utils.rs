use crate::{
    error::{AppError, OutputError, Result, ResultExt},
    tasks::list::TaskList,
};
use colored::Colorize;
use std::io;
use std::io::Write;

pub fn print_tasks_list(tasks: &TaskList, total: usize) -> Result<()> {
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
        writeln!(handle, "{line}")
            .context_transform(|err| AppError::from(OutputError::Io(err)))
            .attach("writing tasks list to stdout")?;
    }
    writeln!(
        handle,
        "⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯\n{}/{} tasks where printed",
        tasks.len(),
        total
    )
    .context_transform(|err| AppError::from(OutputError::Io(err)))
    .attach("writing tasks list to stdout")?;

    handle
        .flush()
        .context_transform(|err| AppError::from(OutputError::Io(err)))
        .attach("flushing stdout")?;

    Ok(())
}
