use chrono::NaiveDate;
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

    #[arg(long, visible_alias = "pri", conflicts_with = "rm_priority")]
    priority: Option<char>,

    #[arg(long, visible_alias = "rm-pri", conflicts_with = "priority")]
    rm_priority: bool,

    #[arg(long, conflicts_with = "rm_due_date")]
    due_date: Option<NaiveDate>,

    #[arg(long, conflicts_with = "due_date")]
    rm_due_date: bool,
}

impl Modify {
    // TODO: https://github.com/just1602/todors/issues/5
    pub fn execute(&self, storage: TaskStorage) -> Result<(), TaskError> {
        let mut tasks = storage.get_all()?;
        let query = TaskQuery::from_string_vec(&self.query)?;

        let idx_to_modify: Vec<usize> = tasks
            .filter_from_query(&query)
            .map(|item| item.idx)
            .collect();

        if self.priority.is_some() {
            tasks.iter_mut().for_each(|item| {
                if idx_to_modify.contains(&item.idx) {
                    item.task.priority = self.priority
                }
            });
        }

        if self.rm_priority {
            tasks.iter_mut().for_each(|item| {
                if idx_to_modify.contains(&item.idx) {
                    item.task.priority = None
                }
            });
        }

        if self.due_date.is_some() {
            tasks.iter_mut().for_each(|item| {
                if idx_to_modify.contains(&item.idx) {
                    item.task.due_date = self.due_date
                }
            });
        }

        if self.rm_due_date {
            tasks.iter_mut().for_each(|item| {
                if idx_to_modify.contains(&item.idx) {
                    item.task.due_date = None
                }
            });
        }

        storage.perist(tasks)
    }
}
