use chrono::{Local, NaiveDate};
use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::tasks::error::TaskError;

// TODO: handle recurrences
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Task {
    pub subject: String,
    pub priority: Option<char>,
    pub created_at: Option<NaiveDate>,
    pub completed_at: Option<NaiveDate>,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
    pub tags: HashMap<String, String>,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.completed {
            f.write_str("x ")?;
            if let Some(completed_at) = self.completed_at {
                f.write_fmt(format_args!("{} ", completed_at))?;
            }
        }

        if self.priority.is_some() && !self.completed {
            f.write_fmt(format_args!("({}) ", self.priority.unwrap()))?;
        }

        if let Some(created_at) = self.created_at {
            f.write_fmt(format_args!("{} ", created_at))?;
        }

        f.write_str(&self.subject)?;

        if let Some(due_date) = self.due_date {
            f.write_fmt(format_args!(" due:{}", due_date))?;
        }

        for (tag, value) in &self.tags {
            f.write_fmt(format_args!(" {}:{}", tag, value))?;
        }

        Ok(())
    }
}

impl FromStr for Task {
    // The original implementation of this trait is highly inspired by this one:
    // https://github.com/kstep/todotxt.rs/blob/master/src/lib.rs

    type Err = TaskError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
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
            TagBegin(usize),
            TagEnd(usize, usize),
        }

        let mut state = State::Init;
        let mut contexts = Vec::new();
        let mut projects = Vec::new();
        let mut tags = HashMap::new();

        // Some tag we know about
        let mut due_date = None;

        for (i, c) in buf.chars().enumerate() {
            let new_state = match (c, state) {
                ('@', State::Init) => State::Context(i),
                ('+', State::Init) => State::Project(i),
                ('A'..='z', State::Init) => State::TagBegin(i),
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
            subject,
            priority,
            created_at,
            completed_at,
            completed,
            due_date,
            contexts,
            projects,
            tags,
        })
    }
}

impl Task {
    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now().date_naive());
        self.priority = None;
    }
}

#[cfg(test)]
mod tests {
    use super::{HashMap, Local, NaiveDate, Task};

    #[test]
    fn it_parses_task() {
        let line = "Some task to do";
        let task = line.parse::<Task>().unwrap();

        assert_eq!(
            task,
            Task {
                subject: "Some task to do".to_string(),
                ..Task::default()
            }
        )
    }

    #[test]
    fn it_parses_task_with_creation_date() {
        let line = "2024-05-01 Some task to do";
        let task = line.parse::<Task>().unwrap();

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
        let task = line.parse::<Task>().unwrap();

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
        let task = line.parse::<Task>().unwrap();

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
        let task = line.parse::<Task>().unwrap();

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
        let task = line.parse::<Task>().unwrap();

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
        let task = line.parse::<Task>().unwrap();

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
        let task = line.parse::<Task>().unwrap();

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
        let task = line.parse::<Task>().unwrap();

        assert_eq!(
            task,
            Task {
                subject: "small".to_string(),
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
