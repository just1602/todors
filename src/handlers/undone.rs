use crate::{
    cli::Undone,
    error::Result,
    storage::TaskStorage,
    tasks::{list::TaskListTrait, query::TaskQuery},
};

pub fn handle_undone(params: Undone, storage: TaskStorage) -> Result<()> {
    let mut tasks = storage.get_all()?;
    let query = TaskQuery::from_string_vec(&params.query)?;

    tasks
        .filter_mut_from_query(&query)
        .for_each(|task| task.undo());

    storage.persist(tasks)
}
