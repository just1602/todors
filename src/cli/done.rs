use crate::tasks::list::{TaskList, TaskListTrait};
use clap::Parser;

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, query::TaskQuery},
};

use super::print_tasks_list;

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

impl Done {
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;
        let total = tasks.len();
        let query = TaskQuery::from_string_vec(&self.query)?;

        tasks
            .filter_mut_from_query(&query)
            .for_each(|item| item.task.complete());

        let completed_tasks: TaskList = tasks.filter_from_query(&query).collect();

        print_tasks_list(&completed_tasks, total);

        storage.perist(tasks)
    }
}
