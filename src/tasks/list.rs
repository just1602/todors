use super::task::Task;

// FIXME: fix task so we can implement `Copy` instead of `Clone`
#[derive(Debug, Clone)]
pub struct TaskListItem {
    pub idx: usize,
    pub task: Task,
}

pub type TaskList = Vec<TaskListItem>;
