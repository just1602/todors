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

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cli::add::handle_add;
use crate::cli::clean::Clean;
use crate::cli::done::handle_done;
use crate::cli::due::Due;
use crate::cli::edit::Edit;
use crate::cli::list::List;
use crate::cli::modify::Modify;
use crate::cli::next::Next;
use crate::cli::remove::Remove;
use crate::cli::undone::Undone;
use crate::config::Config;
use crate::storage::TaskStorage;

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
            Commands::Add(params) => handle_add(params, storage),
            Commands::Done(params) => handle_done(params, storage),
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

#[derive(Parser)]
#[command(name = "add", visible_alias = "a", about = "Add a task to the list")]
pub struct Add {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    task: Vec<String>,

    #[arg(long, help = "Set the priority directly after creating the task")]
    pri: Option<char>,
}

impl Add {
    pub fn new(task: Vec<String>, pri: Option<char>) -> Self {
        Self { task, pri }
    }
}

#[derive(Parser)]
#[command(
    name = "done",
    visible_alias = "do",
    about = "Mark selected tasks as done"
)]
pub struct Done {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,
}
