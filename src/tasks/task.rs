use chrono::{Days, Local, Months, NaiveDate};
use std::{collections::HashMap, fmt::Display};

use crate::tasks::error::TaskError;

// TODO: handle recurrences
// TODO: migrate away from String to &str
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Task {
    pub id: usize,
    pub subject: String,
    pub priority: Option<char>,
    pub created_at: Option<NaiveDate>,
    pub completed_at: Option<NaiveDate>,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
    pub hashtags: Vec<String>,
    pub tags: HashMap<String, String>,
}

// TODO: switch from String to &str
pub struct TaskBuilder {
    id: usize,
    user_query: String,
    pri: Option<char>,
    creation_date: Option<NaiveDate>,
}

impl TaskBuilder {
    pub fn new(id: usize, user_query: String) -> Self {
        TaskBuilder {
            id,
            user_query,
            pri: None,
            creation_date: None,
        }
    }

    pub fn priority(mut self, pri: Option<char>) -> Self {
        self.pri = pri;
        self
    }

    pub fn created_at(mut self, created_at: Option<NaiveDate>) -> Self {
        self.creation_date = created_at;
        self
    }

    pub fn build(self) -> Result<Task, TaskError> {
        let mut task = Task::from_str(self.id, &self.user_query)?;
        if task.created_at.is_none() {
            task.created_at = self.creation_date;
        }

        if task.priority.is_none() {
            task.priority = self.pri;
        }

        Ok(task)
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.completed {
            f.write_str("x ")?;
            if let Some(completed_at) = self.completed_at {
                f.write_fmt(format_args!("{completed_at} "))?;
            }
        }

        if self.priority.is_some() && !self.completed {
            f.write_fmt(format_args!("({}) ", self.priority.unwrap()))?;
        }

        if let Some(created_at) = self.created_at {
            f.write_fmt(format_args!("{created_at} "))?;
        }

        f.write_str(&self.subject)?;

        if let Some(due_date) = self.due_date {
            f.write_fmt(format_args!(" due:{due_date}"))?;
        }

        for (tag, value) in &self.tags {
            f.write_fmt(format_args!(" {tag}:{value}"))?;
        }

        Ok(())
    }
}

impl Task {
    // The original implementation of this trait is highly inspired by this one:
    // https://github.com/kstep/todotxt.rs/blob/master/src/lib.rs

    pub fn from_str(id: usize, mut s: &str) -> Result<Self, TaskError> {
        let (completed, mut completed_at) = if s.starts_with("x ") {
            s = &s[2..];
            (true, s[..10].parse::<NaiveDate>().ok())
        } else {
            (false, None)
        };

        if completed_at.is_some() {
            s = &s[11..];
        }

        let priority = if s.starts_with('(') && &s[2..4] == ") " {
            match s.as_bytes()[1] as char {
                p @ 'A'..='Z' => {
                    s = &s[4..];
                    Some(p)
                }
                _ => None,
            }
        } else {
            None
        };

        let mut created_at = if s.len() < 10 {
            None
        } else if let Ok(date) = s[..10].parse::<NaiveDate>() {
            s = &s[11..];
            Some(date)
        } else {
            None
        };

        // If there's no priority and no completion date in the string, the creation date could be
        // parsed as the completion date, so if it's the case we fix it.
        // TODO: check if there's a cleaner way to refactor the code and avoid this check
        if priority.is_none() && completed_at.is_some() && created_at.is_none() {
            created_at = completed_at;
            completed_at = None;
        }

        let buf = s;

        let mut subject = Vec::new();

        #[derive(Copy, Clone, PartialEq, Eq)]
        enum State {
            Init,
            Context(usize),
            Project(usize),
            HashTag(usize),
            TagBegin(usize),
            TagEnd(usize, usize),
        }

        let mut state = State::Init;
        let mut contexts = Vec::new();
        let mut projects = Vec::new();
        let mut hashtags = Vec::new();
        let mut tags = HashMap::new();

        // Some tag we know about
        let mut due_date = None;

        // NOTE: we must iter on `buf` that way to support non-ascii chars
        let buf_iter = buf.char_indices();
        for (i, c) in buf_iter {
            let new_state = match (c, state) {
                ('@', State::Init) => State::Context(i),
                ('+', State::Init) => State::Project(i),
                ('#', State::Init) => State::HashTag(i),
                (char::MIN..=char::MAX, State::Init) => State::TagBegin(i),
                (':', State::TagBegin(j)) => State::TagEnd(j, i),
                (' ', State::TagBegin(_)) => State::Init,
                (' ', State::Context(j)) => {
                    if i - j > 1 {
                        contexts.push(buf[j + 1..i].to_string());
                    }
                    State::Init
                }
                (' ', State::Project(j)) => {
                    if i - j > 1 {
                        projects.push(buf[j + 1..i].to_string());
                    }
                    State::Init
                }
                (' ', State::HashTag(j)) => {
                    if i - j > 1 {
                        hashtags.push(buf[j + 1..i].to_string());
                    }
                    State::Init
                }
                (' ', State::TagEnd(j, k)) => {
                    if i - k > 1 {
                        match &buf[j..k] {
                            "due" => {
                                due_date = buf[k + 1..i].parse::<NaiveDate>().ok();
                            }
                            tag => {
                                tags.insert(tag.to_string(), buf[k + 1..i].to_string());
                            }
                        }
                    }
                    State::Init
                }
                _ => state,
            };

            if new_state == State::Init {
                match state {
                    State::TagBegin(j) | State::Project(j) | State::Context(j) => {
                        subject.extend(&buf.as_bytes()[j..i + 1]);
                    }
                    State::Init => subject.push(buf.as_bytes()[i]),
                    _ => {}
                }
            }

            state = new_state;
        }

        // parse current tag when we hit EOL
        match state {
            State::TagEnd(j, k) => match &buf[j..k] {
                "due" => {
                    due_date = buf[k + 1..].parse::<NaiveDate>().ok();
                }
                tag => {
                    tags.insert(tag.to_string(), buf[k + 1..].to_string());
                }
            },
            State::Project(j) => {
                projects.push(buf[j + 1..].to_string());
                subject.extend(&buf.as_bytes()[j..]);
            }
            State::Context(j) => {
                contexts.push(buf[j + 1..].to_string());
                subject.extend(&buf.as_bytes()[j..]);
            }
            State::HashTag(j) => {
                hashtags.push(buf[j + 1..].to_string());
                subject.extend(&buf.as_bytes()[j..]);
            }
            State::TagBegin(j) => {
                subject.extend(&buf.as_bytes()[j..]);
            }
            _ => {}
        }

        // FIXME: find a way to not have to use trim_end every time when it's only needed when the
        // string end with a tag.
        let subject = String::from_utf8(subject)
            .unwrap_or_else(|_| s.to_owned())
            .trim_end()
            .to_string();

        Ok(Task {
            id,
            subject,
            priority,
            created_at,
            completed_at,
            completed,
            due_date,
            contexts,
            projects,
            hashtags,
            tags,
        })
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now().date_naive());
        self.priority = None;
    }

    pub fn undo(&mut self) {
        self.completed = false;
        self.completed_at = None;
        // FIXME: if there's a priority, keep it in the description like `pri:value` so we can
        // bring it back
    }

    pub fn compute_urgency(&self) -> i32 {
        // https://taskwarrior.org/docs/urgency/
        // taskwarrior urgency coefficients
        // FIXME: make all those variable configurable
        // urgency.user.tag.next.coefficient           15.0 # +next tag
        // urgency.due.coefficient                     12.0 # overdue or near due date
        // urgency.uda.priority.H.coefficient           6.0 # high Priority
        // urgency.uda.priority.M.coefficient           3.9 # medium Priority
        // urgency.uda.priority.L.coefficient           1.8 # low Priority
        // urgency.age.coefficient                      2.0 # coefficient for age
        // urgency.project.coefficient                  1.0 # assigned to any project
        let mut urgency = 0;

        if self.hashtags.contains(&String::from("next")) {
            urgency += 15;
        }

        if self.hashtags.contains(&String::from("inbox")) {
            urgency += 15;
        }

        if self.hashtags.contains(&String::from("backlog")) {
            urgency -= 30;
        }

        let due_start_to_be_urgent = Local::now()
            .date_naive()
            // FIXME: make the number of days configurable
            .checked_sub_days(Days::new(2))
            .expect("Failed to compute the day when due tasks become urgent.");
        if self
            .due_date
            .is_some_and(|date| date <= due_start_to_be_urgent)
        {
            urgency += 12;
        }

        let pri_urgency = match &self.priority {
            Some('A') => 6,
            Some('B') => 4,
            Some('C') => 2,
            Some('D'..'Z') => 1,
            _ => 0,
        };

        urgency += pri_urgency;

        let one_month_ago = Local::now()
            .date_naive()
            .checked_sub_months(Months::new(1))
            .expect("Failed to compute 1 month ago.");

        if self.created_at.is_some_and(|date| date <= one_month_ago) {
            urgency += 2;
        }

        if !self.projects.is_empty() {
            urgency += 1;
        }

        urgency
    }
}

#[cfg(test)]
mod tests {
    use super::{HashMap, Local, NaiveDate, Task};

    #[test]
    fn it_parses_task() {
        let line = "Some task to do";
        let task = Task::from_str(1, line).unwrap();

        assert_eq!(
            task,
            Task {
                id: 1,
                subject: "Some task to do".to_string(),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_creation_date() {
        let line = "2024-05-01 Some task to do";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do".to_string(),
                created_at: NaiveDate::from_ymd_opt(2024, 05, 01),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_priority_and_creation_date() {
        let line = "(A) 2024-05-01 Some task to do";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do".to_string(),
                priority: Some('A'),
                created_at: NaiveDate::from_ymd_opt(2024, 05, 01),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_completed_task_without_priority_nor_completion_date() {
        let line = "x 2024-05-01 Some task to do";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do".to_string(),
                created_at: NaiveDate::from_ymd_opt(2024, 05, 01),
                completed: true,
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_completed_task_with_completion_date() {
        let line = "x 2024-06-01 2024-05-01 Some task to do";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do".to_string(),
                created_at: NaiveDate::from_ymd_opt(2024, 05, 01),
                completed_at: NaiveDate::from_ymd_opt(2024, 06, 01),
                completed: true,
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_project_and_context() {
        let line = "2024-05-01 Some task to do +project @home";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do +project @home".to_string(),
                created_at: NaiveDate::from_ymd_opt(2024, 5, 1),
                projects: vec!["project".to_string()],
                contexts: vec!["home".to_string()],
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_tags() {
        let line = "2024-05-01 Some task to do +project @home due:2024-06-01 team:devops";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do +project @home".to_string(),
                created_at: NaiveDate::from_ymd_opt(2024, 5, 1),
                projects: vec!["project".to_string()],
                contexts: vec!["home".to_string()],
                due_date: NaiveDate::from_ymd_opt(2024, 6, 1),
                tags: HashMap::from([("team".to_string(), "devops".to_string())]),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_due_date_at_the_end() {
        let line = "2024-05-01 Some task to do +project @home team:devops due:2024-06-01";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do +project @home".to_string(),
                created_at: NaiveDate::from_ymd_opt(2024, 5, 1),
                projects: vec!["project".to_string()],
                contexts: vec!["home".to_string()],
                due_date: NaiveDate::from_ymd_opt(2024, 6, 1),
                tags: HashMap::from([("team".to_string(), "devops".to_string())]),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_small_task() {
        let line = "small";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "small".to_string(),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_french_accent_correctly() {
        let line = "2024-05-01 écrire une tâche avec des accents due:2024-06-01";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "écrire une tâche avec des accents".to_string(),
                created_at: NaiveDate::from_ymd_opt(2024, 5, 1),
                projects: vec![],
                contexts: vec![],
                due_date: NaiveDate::from_ymd_opt(2024, 6, 1),
                tags: HashMap::from([]),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_hash_tag() {
        let line = "some task I'll do #next";
        let task = Task::from_str(0, line).unwrap();

        assert_eq!(
            task,
            Task {
                subject: "some task I'll do #next".to_string(),
                hashtags: vec!["next".to_string()],
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_display_task() {
        let task = Task {
            subject: "Some task to do".to_string(),
            ..Task::default()
        };

        let result = format!("{}", task);

        assert_eq!(result, "Some task to do");
    }

    #[test]
    fn it_display_task_with_priority() {
        let task = Task {
            subject: "Some task to do".to_string(),
            priority: Some('A'),
            ..Task::default()
        };

        let result = format!("{}", task);

        assert_eq!(result, "(A) Some task to do");
    }

    #[test]
    fn it_display_completed_task() {
        let task = Task {
            subject: "Some task to do".to_string(),
            completed: true,
            priority: Some('A'),
            ..Task::default()
        };

        let result = format!("{}", task);

        assert_eq!(result, "x Some task to do");
    }

    #[test]
    fn it_display_completed_task_with_completion_date() {
        let task = Task {
            subject: "Some task to do".to_string(),
            completed_at: NaiveDate::from_ymd_opt(2024, 6, 1),
            completed: true,
            priority: Some('A'),
            ..Task::default()
        };

        let result = format!("{}", task);

        assert_eq!(result, "x 2024-06-01 Some task to do");
    }

    #[test]
    fn it_display_completed_task_with_creation_and_completion_dates() {
        let task = Task {
            subject: "Some task to do".to_string(),
            created_at: NaiveDate::from_ymd_opt(2024, 5, 1),
            completed_at: NaiveDate::from_ymd_opt(2024, 6, 1),
            completed: true,
            priority: Some('A'),
            ..Task::default()
        };

        let result = format!("{}", task);

        assert_eq!(result, "x 2024-06-01 2024-05-01 Some task to do");
    }

    #[test]
    fn it_display_task_with_due_date_and_tags() {
        let task = Task {
            subject: "Some task to do".to_string(),
            due_date: NaiveDate::from_ymd_opt(2024, 6, 1),
            priority: Some('A'),
            tags: HashMap::from([("team".to_string(), "devops".to_string())]),
            ..Task::default()
        };

        let result = format!("{}", task);

        assert_eq!(result, "(A) Some task to do due:2024-06-01 team:devops");
    }

    #[test]
    fn it_complete_task_properly() {
        let mut task = Task {
            subject: "Some task to do".to_string(),
            due_date: NaiveDate::from_ymd_opt(2024, 6, 1),
            priority: Some('A'),
            tags: HashMap::from([("team".to_string(), "devops".to_string())]),
            ..Task::default()
        };

        task.complete();

        assert_eq!(task.priority, None);
        assert_eq!(task.completed_at, Some(Local::now().date_naive()));
        assert!(task.completed);
    }
}
