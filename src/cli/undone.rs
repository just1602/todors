use clap::Parser;

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListVecExt, query::TaskQuery},
};

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

impl Undone {
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;
        let query = TaskQuery::from_string_vec(self.query.clone())?;

        tasks
            .filter_mut_from_query(&query)
            .for_each(|item| item.task.undo());

        storage.perist(tasks)
    }
}
