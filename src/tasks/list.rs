use chrono::{Days, Local};

use super::{query::TaskQuery, task::Task};

pub type TaskList = Vec<Task>;

pub trait TaskListTrait {
    fn filter_from_query(&self, query: &TaskQuery) -> impl Iterator<Item = Task>;
    fn filter_mut_from_query(&mut self, query: &TaskQuery) -> impl Iterator<Item = &mut Task>;
    fn sort_by_urgency(&self) -> TaskList;
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

    fn sort_by_urgency(&self) -> TaskList {
        // FIXME: For each task compute an urgency indices and sort on the indices.
        let mut due_tasks: TaskList = self
            .clone()
            .into_iter()
            .filter(|item| {
                let due_start_to_be_urgent = Local::now()
                    .date_naive()
                    .checked_sub_days(Days::new(2))
                    .expect("Failed to compute the day when due tasks become urgent.");
                item.due_date.is_some_and(|d| d <= due_start_to_be_urgent)
            })
            .collect();
        due_tasks.sort_by_key(|item| item.due_date);
        let mut pri_tasks: TaskList = self
            .clone()
            .into_iter()
            .filter(|item| item.priority.is_some())
            .collect();
        pri_tasks.sort_by_key(|item| item.priority);

        let mut other_tasks: TaskList = self
            .clone()
            .into_iter()
            .filter(|item| item.priority.is_none())
            .collect();
        other_tasks.sort_by_key(|item| item.id);

        let mut tasks: TaskList = Vec::new();
        [due_tasks, pri_tasks, other_tasks]
            .concat()
            .iter()
            .for_each(|item| {
                if !tasks.contains(item) {
                    tasks.push(item.clone());
                }
            });
        tasks.retain(|item| !item.completed);
        tasks
    }
}
