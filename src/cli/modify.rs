use clap::Parser;

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, list::TaskListTrait, query::TaskQuery},
};

#[derive(Parser)]
#[command(
    name = "modify",
    visible_alias = "mod",
    about = "Modify selected tasks as desired"
)]
pub struct Modify {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,

    #[arg(long, short, visible_alias = "pri")]
    priority: Option<char>,
}

impl Modify {
    // TODO: https://github.com/just1602/todors/issues/5
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;
        let query = TaskQuery::from_string_vec(self.query.clone())?;

        tasks
            .filter_mut_from_query(&query)
            .for_each(|item| item.task.priority = self.priority);

        storage.perist(tasks)
    }
}
