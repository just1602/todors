mod handlers;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use handlers::{
    handle_add, handle_clean, handle_done, handle_due, handle_edit, handle_list, handle_modify,
    handle_remove, handle_undone,
};

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
    pub fn run(self, config: Config, storage: impl TaskStorage) {
        let result = match self.command {
            Commands::Add(params) => handle_add(config, params),
            Commands::Done(params) => handle_done(storage, params),
            Commands::List(params) => handle_list(storage, params),
            Commands::Remove(params) => handle_remove(storage, params),
            Commands::Edit(params) => handle_edit(config, params),
            Commands::Due => handle_due(storage),
            Commands::Undone(params) => handle_undone(storage, params),
            Commands::Clean => handle_clean(storage),
            Commands::Modify(params) => handle_modify(storage, params),
        };

        if let Err(e) = result {
            eprintln!("An error occured: {}", e);
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Add(AddParams),
    Done(DoneParams),
    List(ListParams),
    Remove(RemoveParams),
    Edit(EditParams),
    Due,
    Undone(UndoneParams),
    Clean,
    Modify(ModifyParams),
}

#[derive(Parser)]
#[command(name = "add", visible_alias = "a", about = "Add a task to the list")]
struct AddParams {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    task: Vec<String>,

    #[arg(long, help = "Set the priority directly after creating the task")]
    pri: Option<char>,
}

#[derive(Parser)]
#[command(
    name = "list",
    visible_alias = "ls",
    about = "List all the tasks or those that match the query"
)]
struct ListParams {
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
    name = "done",
    visible_alias = "do",
    about = "Mark selected tasks as done"
)]
struct DoneParams {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,
}

#[derive(Parser)]
#[command(
    name = "remove",
    visible_alias = "rm",
    about = "Remove selected item from the todo file"
)]
struct RemoveParams {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,
}

#[derive(Parser)]
#[command(name = "edit", about = "Edit the todo file with a text editor")]
struct EditParams {
    item: Option<u32>,
}

#[derive(Parser)]
#[command(
    name = "undone",
    visible_alias = "undo",
    about = "Mark selected tasks as not done"
)]
struct UndoneParams {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,
}

#[derive(Parser)]
#[command(
    name = "modify",
    visible_alias = "mod",
    about = "Modify selected tasks as desired"
)]
struct ModifyParams {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,

    #[arg(long, short, visible_alias = "pri")]
    priority: Option<char>,
}
