use super::{query::TaskQuery, task::Task};

pub type TaskList = Vec<Task>;

pub trait TaskListTrait {
    fn filter_from_query(&self, query: &TaskQuery) -> impl Iterator<Item = Task>;
    fn filter_mut_from_query(&mut self, query: &TaskQuery) -> impl Iterator<Item = &mut Task>;
    fn sort_by_urgency(&mut self);
}

impl TaskListTrait for TaskList {
    fn filter_from_query(&self, query: &TaskQuery) -> impl Iterator<Item = Task> {
        self.iter().filter(|task| query.matches(task)).cloned()
    }

    fn filter_mut_from_query(&mut self, query: &TaskQuery) -> impl Iterator<Item = &mut Task> {
        self.iter_mut().filter(|task| query.matches(task))
    }

    fn sort_by_urgency(&mut self) {
        self.sort_by_key(|task| task.compute_urgency());
        self.reverse();
    }
}
