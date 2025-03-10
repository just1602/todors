use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

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
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
    pub task: Vec<String>,

    #[arg(long, help = "Set the priority directly after creating the task")]
    pub pri: Option<char>,
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
    pub query: Vec<String>,
}

#[derive(Parser)]
#[command(
    name = "list",
    visible_alias = "ls",
    about = "List all the tasks or those that match the query"
)]
pub struct List {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub query: Option<Vec<String>>,

    #[arg(
        long,
        help = "Display all tasks, even the completed ones",
        default_value_t = false
    )]
    pub all: bool,
}

#[derive(Parser)]
#[command(
    name = "remove",
    visible_alias = "rm",
    about = "Remove selected item from the todo file"
)]
pub struct Remove {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    pub query: Vec<String>,
}

#[derive(Parser)]
#[command(name = "edit", about = "Edit the todo file with a text editor")]
pub struct Edit {
    pub item: Option<u32>,
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
    pub query: Vec<String>,
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
    pub query: Vec<String>,

    #[arg(long, visible_alias = "pri", conflicts_with = "rm_priority")]
    pub priority: Option<char>,

    #[arg(long, visible_alias = "rm-pri", conflicts_with = "priority")]
    pub rm_priority: bool,

    #[arg(long, conflicts_with = "rm_due_date")]
    pub due_date: Option<NaiveDate>,

    #[arg(long, conflicts_with = "due_date")]
    pub rm_due_date: bool,
}

#[derive(Parser)]
#[command(
    name = "next",
    about = "Show the next task to do base on the urgency task sort we have"
)]
pub struct Next {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub query: Option<Vec<String>>,
}
