use crate::{
    cli::Done,
    error::{AppError, AppResult, ResultExt},
    storage::TaskStorage,
    tasks::query::TaskQuery,
    utils::print_tasks_list,
};

pub fn handle_done(params: Done, storage: TaskStorage) -> AppResult<()> {
    let mut tasks = storage.get_all().context_to::<AppError>()?;
    let total = tasks.len();
    let query = TaskQuery::from_string_vec(&params.query).context_to::<AppError>()?;

    tasks
        .iter_mut()
        .filter(|task| query.matches(task))
        .for_each(|task| task.complete());

    print_tasks_list(tasks.iter().filter(|task| query.matches(task)), total)?;

    storage.persist(tasks).context_to::<AppError>()
}
