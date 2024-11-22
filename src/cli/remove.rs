use clap::Parser;

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

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

impl Remove {
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let tasks = storage.get_all()?;
        let query = TaskQuery::from_string_vec(&self.query)?;

        let idx_to_remove: Vec<usize> = tasks
            .filter_from_query(&query)
            .map(|item| item.idx)
            .collect();

        let tasks = tasks
            .into_iter()
            .filter(|item| !idx_to_remove.contains(&item.idx))
            .collect();

        storage.perist(tasks)
    }
}
