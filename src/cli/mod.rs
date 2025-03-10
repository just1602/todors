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

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use crate::cli::add::handle_add;
use crate::cli::clean::handle_clean;
use crate::cli::done::handle_done;
use crate::cli::due::handle_due;
use crate::cli::edit::handle_edit;
use crate::cli::list::handle_list;
use crate::cli::modify::handle_modify;
use crate::cli::next::handle_next;
use crate::cli::remove::handle_remove;
use crate::cli::undone::handle_undone;
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
            Commands::List(params) => handle_list(params, storage),
            Commands::Remove(params) => handle_remove(params, storage),
            Commands::Edit(params) => handle_edit(params, config),
            Commands::Due(params) => handle_due(params, storage),
            Commands::Undone(params) => handle_undone(params, storage),
            Commands::Clean(params) => handle_clean(params, storage),
            Commands::Modify(params) => handle_modify(params, storage),
            Commands::Next(params) => handle_next(params, storage),
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

#[derive(Parser)]
#[command(
    name = "list",
    visible_alias = "ls",
    about = "List all the tasks or those that match the query"
)]
pub struct List {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    query: Option<Vec<String>>,

    #[arg(
        long,
        help = "Display all tasks, even the completed ones",
        default_value_t = false
    )]
    all: bool,
}

#[derive(Parser)]
#[command(
    name = "remove",
    visible_alias = "rm",
    about = "Remove selected item from the todo file"
)]
pub struct Remove {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,
}

#[derive(Parser)]
#[command(name = "edit", about = "Edit the todo file with a text editor")]
pub struct Edit {
    item: Option<u32>,
}

#[derive(Parser)]
#[command(name = "due", about = "List all due tasks")]
pub struct Due;

#[derive(Parser)]
#[command(
    name = "undone",
    visible_alias = "undo",
    about = "Mark selected tasks as not done"
)]
pub struct Undone {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,
}

#[derive(Parser)]
#[command(name = "clean", about = "Clean all the completed tasks")]
pub struct Clean;

#[derive(Parser)]
#[command(
    name = "modify",
    visible_alias = "mod",
    about = "Modify selected tasks as desired"
)]
pub struct Modify {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,

    #[arg(long, visible_alias = "pri", conflicts_with = "rm_priority")]
    priority: Option<char>,

    #[arg(long, visible_alias = "rm-pri", conflicts_with = "priority")]
    rm_priority: bool,

    #[arg(long, conflicts_with = "rm_due_date")]
    due_date: Option<NaiveDate>,

    #[arg(long, conflicts_with = "due_date")]
    rm_due_date: bool,
}

#[derive(Parser)]
#[command(
    name = "next",
    about = "Show the next task to do base on the urgency task sort we have"
)]
pub struct Next {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    query: Option<Vec<String>>,
}
