use crate::{
    cli::Done,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::{
        list::{TaskList, TaskListTrait},
        query::TaskQuery,
    },
    utils::print_tasks_list,
};

pub fn handle_done(params: Done, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;
    let total = tasks.len();
    let query = TaskQuery::from_string_vec(&params.query).context_to::<AppError>()?;

    tasks
        .filter_mut_from_query(&query)
        .for_each(|task| task.complete());

    let completed_tasks: TaskList = tasks.filter_from_query(&query).collect();

    print_tasks_list(&completed_tasks, total)?;

    storage.persist(tasks).context_to::<AppError>()
}
