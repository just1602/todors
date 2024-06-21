use super::task::Task;

#[derive(Debug)]
pub struct TaskListItem {
    pub idx: usize,
    pub task: Task,
}

pub type TaskList = Vec<TaskListItem>;
