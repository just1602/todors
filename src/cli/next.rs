use clap::Parser;

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

use super::print_tasks_list;

#[derive(Parser)]
#[command(
    name = "next",
    about = "Show the next task to do base on the urgency task sort we have"
)]
pub struct Next {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    query: Option<Vec<String>>,
}

impl Next {
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;
        let total = tasks.len();

        if let Some(query) = &self.query {
            let query = TaskQuery::from_string_vec(query)?;

            tasks = tasks.filter_from_query(&query).collect();
        }

        if let Some(task) = tasks.sort_by_urgency().first() {
            // FIXME: remove this clone
            // TODO: check if this function can take a slice instead
            print_tasks_list(&vec![task.clone()], total)?;
        }

        Ok(())
    }
}
