pub mod add;
mod clean;
mod done;
mod due;
mod edit;
mod list;
mod modify;
mod next;
mod remove;
mod undone;

use std::io;
use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cli::add::Add;
use crate::cli::clean::Clean;
use crate::cli::done::Done;
use crate::cli::due::Due;
use crate::cli::edit::Edit;
use crate::cli::list::List;
use crate::cli::modify::Modify;
use crate::cli::next::Next;
use crate::cli::remove::Remove;
use crate::cli::undone::Undone;
use crate::config::Config;
use crate::storage::TaskStorage;
use crate::tasks::error::TaskError;
use crate::tasks::list::{TaskList, TaskListTrait};

use colored::Colorize;

#[derive(Parser)]
#[command(
    version,
    about = "todors - CLI todo app using the todo.txt format.",
    long_about = None)
]
pub struct Cli {
    #[arg(long = "config", short = 'c', help = "Path to the config file.")]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(self, config: Config, storage: TaskStorage) {
        let result = match self.command {
            Commands::Add(add) => add.execute(storage),
            Commands::Done(done) => done.execute(storage),
            Commands::List(list) => list.execute(storage),
            Commands::Remove(remove) => remove.execute(storage),
            Commands::Edit(edit) => edit.execute(config),
            Commands::Due(due) => due.execute(storage),
            Commands::Undone(undone) => undone.execute(storage),
            Commands::Clean(clean) => clean.execute(storage),
            Commands::Modify(modify) => modify.execute(storage),
            Commands::Next(next) => next.execute(storage),
        };

        if let Err(e) = result {
            eprintln!("An error occured: {}", e);
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Add(Add),
    Done(Done),
    List(List),
    Remove(Remove),
    Edit(Edit),
    Due(Due),
    Undone(Undone),
    Clean(Clean),
    Modify(Modify),
    Next(Next),
}

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
