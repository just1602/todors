use std::cmp::Reverse;

use super::task::Task;

pub type TaskList = Vec<Task>;

pub trait TaskListTrait {
    fn sort_by_urgency(&mut self);
}

impl TaskListTrait for TaskList {
    fn sort_by_urgency(&mut self) {
        self.sort_by_key(|task| Reverse(task.compute_urgency()));
    }
}

#[cfg(test)]
mod tests {
    use super::{TaskList, TaskListTrait};
    use crate::tasks::{strategies::urgency_task, task::Task};
    use proptest::prelude::*;

    fn task(id: usize, priority: Option<char>) -> Task {
        Task {
            id,
            subject: format!("task {id}"),
            priority,
            ..Task::default()
        }
    }

    fn ids(tasks: &TaskList) -> Vec<usize> {
        tasks.iter().map(|task| task.id).collect()
    }

    #[test]
    fn it_sorts_by_descending_urgency() {
        let mut tasks: TaskList = vec![task(1, None), task(2, Some('B')), task(3, Some('A'))];

        tasks.sort_by_urgency();

        assert_eq!(ids(&tasks), vec![3, 2, 1]);
    }

    #[test]
    fn it_keeps_file_order_for_equal_urgency() {
        let mut tasks: TaskList = vec![task(1, Some('A')), task(2, Some('A')), task(3, Some('A'))];

        tasks.sort_by_urgency();

        assert_eq!(ids(&tasks), vec![1, 2, 3]);
    }

    proptest! {
        #[test]
        fn urgency_sort_is_stable_and_ordered(
            generated in prop::collection::vec(urgency_task(), 0..25)
        ) {
            let original: Vec<(usize, i32)> = generated
                .iter()
                .map(|task| (task.id, task.compute_urgency()))
                .collect();
            let mut expected = original.clone();
            expected.sort_by_key(|&(_, urgency)| std::cmp::Reverse(urgency));

            let mut tasks = generated;
            tasks.sort_by_urgency();
            let actual: Vec<(usize, i32)> = tasks
                .iter()
                .map(|task| (task.id, task.compute_urgency()))
                .collect();

            prop_assert_eq!(actual, expected);
        }
    }
}
