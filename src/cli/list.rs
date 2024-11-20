use clap::Parser;

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

use super::print_tasks_list;

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

impl List {
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;
        let total = tasks.len();

        if !self.all {
            tasks.retain(|item| !item.task.completed)
        }

        if let Some(query) = &self.query {
            let query = TaskQuery::from_string_vec(query.clone())?;

            tasks = tasks.filter_from_query(&query).collect();
        }

        print_tasks_list(tasks, total);
        Ok(())
    }
}
