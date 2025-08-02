use crate::cli::Done;
use crate::tasks::list::{TaskList, TaskListTrait};

use crate::{
    storage::TaskStorage,
    tasks::{error::TaskError, query::TaskQuery},
};

use crate::utils::print_tasks_list;

pub fn handle_done(params: Done, storage: TaskStorage) -> Result<(), TaskError> {
    let mut tasks = storage.get_all()?;
    let total = tasks.len();
    let query = TaskQuery::from_string_vec(&params.query)?;

    tasks
        .filter_mut_from_query(&query)
        .for_each(|task| task.complete());

    let completed_tasks: TaskList = tasks.filter_from_query(&query).collect();

    print_tasks_list(&completed_tasks, total)?;

    storage.persist(tasks)
}
