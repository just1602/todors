pub mod add;
mod handlers;
pub mod list;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use handlers::{
    handle_clean, handle_done, handle_due, handle_edit, handle_modify, handle_remove, handle_undone,
};

use crate::cli::add::Add;
use crate::cli::list::List;
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
            Commands::Add(add) => add.execute(storage),
            Commands::Done(params) => handle_done(storage, params),
            Commands::List(list) => list.execute(storage),
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
    Add(Add),
    Done(DoneParams),
    List(List),
    Remove(RemoveParams),
    Edit(EditParams),
    Due,
    Undone(UndoneParams),
    Clean,
    Modify(ModifyParams),
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
