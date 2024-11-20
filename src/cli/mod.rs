pub mod add;
pub mod clean;
pub mod done;
pub mod due;
pub mod edit;
pub mod list;
pub mod modify;
pub mod remove;
pub mod undone;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cli::add::Add;
use crate::cli::clean::Clean;
use crate::cli::done::Done;
use crate::cli::due::Due;
use crate::cli::edit::Edit;
use crate::cli::list::List;
use crate::cli::modify::Modify;
use crate::cli::remove::Remove;
use crate::cli::undone::Undone;
use crate::config::Config;
use crate::storage::TaskStorage;
use crate::tasks::list::TaskList;

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
}

pub fn print_tasks_list(tasks: TaskList, total: usize) {
    // FIXME: there must be a better way to do this sort
    let mut pri_tasks: TaskList = tasks
        .clone()
        .into_iter()
        .filter(|item| item.task.priority.is_some())
        .collect();
    pri_tasks.sort_by_key(|item| item.task.priority);

    let mut other_tasks: TaskList = tasks
        .clone()
        .into_iter()
        .filter(|item| item.task.priority.is_none())
        .collect();

    other_tasks.sort_by_key(|item| item.idx);

    let tasks = [pri_tasks, other_tasks].concat();

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
