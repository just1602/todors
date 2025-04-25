use super::{query::TaskQuery, task::Task};

pub type TaskList = Vec<Task>;

pub trait TaskListTrait {
    fn filter_from_query(&self, query: &TaskQuery) -> impl Iterator<Item = Task>;
    fn filter_mut_from_query(&mut self, query: &TaskQuery) -> impl Iterator<Item = &mut Task>;
    fn sort_by_urgency(&mut self) -> TaskList;
}

impl TaskListTrait for TaskList {
    fn filter_from_query(&self, query: &TaskQuery) -> impl Iterator<Item = Task> {
        self.iter()
            .filter(|item| {
                if query.indexes.contains(&item.id) {
                    return true;
                }

                if item.projects.iter().any(|pro| query.projects.contains(pro)) {
                    return true;
                }

                if item.contexts.iter().any(|ctx| query.contexts.contains(ctx)) {
                    return true;
                }

                if item.hashtags.iter().any(|ctx| query.hashtags.contains(ctx)) {
                    return true;
                }

                if let Some(due_date) = query.due_date {
                    if item.due_date.is_some_and(|dd| dd == due_date) {
                        return true;
                    }
                }

                if !query.subject.is_empty() && item.subject.contains(&query.subject) {
                    return true;
                }

                false
            })
            .cloned()
    }

    fn filter_mut_from_query(&mut self, query: &TaskQuery) -> impl Iterator<Item = &mut Task> {
        self.iter_mut().filter(|item| {
            if query.indexes.contains(&item.id) {
                return true;
            }

            if item.projects.iter().any(|pro| query.projects.contains(pro)) {
                return true;
            }

            if item.contexts.iter().any(|ctx| query.contexts.contains(ctx)) {
                return true;
            }

            if item.hashtags.iter().any(|ctx| query.hashtags.contains(ctx)) {
                return true;
            }

            if let Some(due_date) = query.due_date {
                if item.due_date.is_some_and(|dd| dd == due_date) {
                    return true;
                }
            }

            // FIXME: add tests for this, and make sure to add a test that check for empty subject
            if !query.subject.is_empty() && item.subject.contains(&query.subject) {
                return true;
            }

            false
        })
    }

    fn sort_by_urgency(&mut self) -> TaskList {
        self.sort_by_key(|task| task.compute_urgency());
        self.reverse();
        self.to_vec()
    }
}
