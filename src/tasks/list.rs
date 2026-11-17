use super::task::Task;

pub type TaskList = Vec<Task>;

pub trait TaskListTrait {
    fn sort_by_urgency(&mut self);
}

impl TaskListTrait for TaskList {
    fn sort_by_urgency(&mut self) {
        self.sort_by_key(|task| task.compute_urgency());
        self.reverse();
    }
}
