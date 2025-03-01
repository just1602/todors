use super::{query::TaskQuery, task::Task};

// FIXME: fix task so we can implement `Copy` instead of `Clone`
#[derive(Debug, Clone)]
pub struct TaskListItem {
    pub idx: usize,
    pub task: Task,
}

pub type TaskList = Vec<TaskListItem>;

pub trait TaskListTrait {
    fn filter_from_query(&self, query: &TaskQuery) -> impl Iterator<Item = TaskListItem>;
    fn filter_mut_from_query(
        &mut self,
        query: &TaskQuery,
    ) -> impl Iterator<Item = &mut TaskListItem>;
    fn sort_by_urgency(&self) -> TaskList;
}

impl TaskListTrait for TaskList {
    fn filter_from_query(&self, query: &TaskQuery) -> impl Iterator<Item = TaskListItem> {
        self.iter()
            .filter(|item| {
                if query.indexes.contains(&item.idx) {
                    return true;
                }

                if item
                    .task
                    .projects
                    .iter()
                    .any(|pro| query.projects.contains(pro))
                {
                    return true;
                }

                if item
                    .task
                    .contexts
                    .iter()
                    .any(|ctx| query.contexts.contains(ctx))
                {
                    return true;
                }

                if let Some(due_date) = query.due_date {
                    if item.task.due_date.is_some_and(|dd| dd == due_date) {
                        return true;
                    }
                }

                if !query.subject.is_empty() && item.task.subject.contains(&query.subject) {
                    return true;
                }

                false
            })
            .cloned()
    }

    fn filter_mut_from_query(
        &mut self,
        query: &TaskQuery,
    ) -> impl Iterator<Item = &mut TaskListItem> {
        self.iter_mut().filter(|item| {
            if query.indexes.contains(&item.idx) {
                return true;
            }

            if item
                .task
                .projects
                .iter()
                .any(|pro| query.projects.contains(pro))
            {
                return true;
            }

            if item
                .task
                .contexts
                .iter()
                .any(|ctx| query.contexts.contains(ctx))
            {
                return true;
            }

            if let Some(due_date) = query.due_date {
                if item.task.due_date.is_some_and(|dd| dd == due_date) {
                    return true;
                }
            }

            // FIXME: add tests for this, and make sure to add a test that check for empty subject
            if !query.subject.is_empty() && item.task.subject.contains(&query.subject) {
                return true;
            }

            false
        })
    }

    fn sort_by_urgency(&self) -> TaskList {
        // FIXME: there must be a better way to do this sort
        let mut pri_tasks: TaskList = self
            .clone()
            .into_iter()
            .filter(|item| item.task.priority.is_some())
            .collect();
        pri_tasks.sort_by_key(|item| item.task.priority);

        let mut other_tasks: TaskList = self
            .clone()
            .into_iter()
            .filter(|item| item.task.priority.is_none())
            .collect();

        other_tasks.sort_by_key(|item| item.idx);

        [pri_tasks, other_tasks].concat()
    }
}
