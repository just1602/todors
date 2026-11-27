use chrono::{Days, Local, Months, NaiveDate};
use std::{collections::HashMap, fmt::Display};

use crate::error::{TaskError, TaskResult, bail, report};

// TODO: handle recurrences
// TODO: migrate away from String to &str
#[derive(Debug, Eq, PartialEq, Default)]
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

    pub fn build(self) -> TaskResult<Task> {
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

        if let Some(priority) = self.priority.filter(|_| !self.completed) {
            f.write_fmt(format_args!("({priority}) "))?;
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

const TODO_DATE_LEN: usize = 10;

fn split_date_prefix(s: &str) -> Option<(NaiveDate, &str)> {
    let date_text = s.get(..TODO_DATE_LEN)?;
    let rest = s.get(TODO_DATE_LEN..)?.strip_prefix(' ')?;
    let date = date_text.parse::<NaiveDate>().ok()?;
    Some((date, rest))
}

fn split_priority(s: &str, line: usize) -> TaskResult<Option<(char, &str)>> {
    let Some(after_open) = s.strip_prefix('(') else {
        return Ok(None);
    };

    let mut chars = after_open.chars();
    let Some(priority) = chars.next() else {
        return Ok(None);
    };

    if chars.next() != Some(')') || chars.next() != Some(' ') {
        return Ok(None);
    }

    if !priority.is_ascii_uppercase() {
        bail!(TaskError::InvalidPriority(priority));
    }

    let Some(rest) = s.get(4..) else {
        bail!(TaskError::ParseTask {
            line,
            detail: "invalid byte boundary after priority".to_owned(),
        });
    };

    Ok(Some((priority, rest)))
}

fn task_slice(source: &str, start: usize, end: usize, line: usize) -> TaskResult<&str> {
    source.get(start..end).ok_or_else(|| {
        report!(TaskError::ParseTask {
            line,
            detail: "invalid byte boundary while parsing task".to_owned(),
        })
    })
}

fn append_task_slice(
    subject: &mut String,
    source: &str,
    start: usize,
    end: usize,
    line: usize,
) -> TaskResult<()> {
    subject.push_str(task_slice(source, start, end, line)?);
    Ok(())
}

impl Task {
    pub fn from_str(id: usize, mut s: &str) -> TaskResult<Self> {
        let (completed, mut completed_at) = if let Some(after_x) = s.strip_prefix("x ") {
            match split_date_prefix(after_x) {
                Some((date, rest)) => {
                    s = rest;
                    (true, Some(date))
                }
                None => {
                    s = after_x;
                    (true, None)
                }
            }
        } else {
            (false, None)
        };

        let priority = match split_priority(s, id)? {
            Some((priority, rest)) => {
                s = rest;
                Some(priority)
            }
            None => None,
        };

        let mut created_at = match split_date_prefix(s) {
            Some((date, rest)) => {
                s = rest;
                Some(date)
            }
            None => None,
        };

        // If there's no priority and no completion date in the string, the creation date could be
        // parsed as the completion date, so if it's the case we fix it.
        // TODO: check if there's a cleaner way to refactor the code and avoid this check
        if priority.is_none() && completed_at.is_some() && created_at.is_none() {
            created_at = completed_at;
            completed_at = None;
        }

        let buf = s;

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
        let mut subject = String::new();

        // Some tag we know about
        let mut due_date = None;

        for (i, c) in buf.char_indices() {
            let char_end = i + c.len_utf8();
            let new_state = match (c, state) {
                ('@', State::Init) => State::Context(i),
                ('+', State::Init) => State::Project(i),
                ('#', State::Init) => State::HashTag(i),
                (char::MIN..=char::MAX, State::Init) => State::TagBegin(i),
                (':', State::TagBegin(j)) => State::TagEnd(j, i),
                (' ', State::TagBegin(_)) => State::Init,
                (' ', State::Context(j)) => {
                    if i > j + 1 {
                        contexts.push(task_slice(buf, j + 1, i, id)?.to_string());
                    }
                    State::Init
                }
                (' ', State::Project(j)) => {
                    if i > j + 1 {
                        projects.push(task_slice(buf, j + 1, i, id)?.to_string());
                    }
                    State::Init
                }
                (' ', State::HashTag(j)) => {
                    if i > j + 1 {
                        hashtags.push(task_slice(buf, j + 1, i, id)?.to_string());
                    }
                    State::Init
                }
                (' ', State::TagEnd(j, k)) => {
                    if i > k + 1 {
                        let tag = task_slice(buf, j, k, id)?;
                        let value = task_slice(buf, k + 1, i, id)?;
                        if tag == "due" {
                            due_date = value.parse::<NaiveDate>().ok();
                        } else {
                            tags.insert(tag.to_string(), value.to_string());
                        }
                    }
                    State::Init
                }
                _ => state,
            };

            if new_state == State::Init {
                match state {
                    State::TagBegin(j)
                    | State::Project(j)
                    | State::Context(j)
                    | State::HashTag(j) => {
                        append_task_slice(&mut subject, buf, j, char_end, id)?;
                    }
                    State::Init => subject.push(c),
                    _ => {}
                }
            }

            state = new_state;
        }

        match state {
            State::TagEnd(j, k) if buf.len() > k + 1 => {
                let tag = task_slice(buf, j, k, id)?;
                let value = task_slice(buf, k + 1, buf.len(), id)?;
                if tag == "due" {
                    due_date = value.parse::<NaiveDate>().ok();
                } else {
                    tags.insert(tag.to_string(), value.to_string());
                }
            }
            State::Project(j) => {
                if buf.len() > j + 1 {
                    projects.push(task_slice(buf, j + 1, buf.len(), id)?.to_string());
                }
                append_task_slice(&mut subject, buf, j, buf.len(), id)?;
            }
            State::Context(j) => {
                if buf.len() > j + 1 {
                    contexts.push(task_slice(buf, j + 1, buf.len(), id)?.to_string());
                }
                append_task_slice(&mut subject, buf, j, buf.len(), id)?;
            }
            State::HashTag(j) => {
                if buf.len() > j + 1 {
                    hashtags.push(task_slice(buf, j + 1, buf.len(), id)?.to_string());
                }
                append_task_slice(&mut subject, buf, j, buf.len(), id)?;
            }
            State::TagBegin(j) => {
                append_task_slice(&mut subject, buf, j, buf.len(), id)?;
            }
            _ => {}
        }

        let subject = subject.trim_end().to_string();
        if subject.is_empty() {
            bail!(TaskError::EmptySubject);
        }

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
        if let Some(priority) = self.priority {
            self.tags.insert("pri".to_owned(), priority.to_string());
        }

        self.completed = true;
        self.completed_at = Some(Local::now().date_naive());
        self.priority = None;
    }

    pub fn undo(&mut self) {
        self.completed = false;
        self.completed_at = None;

        if let Some(priority) = self.tags.remove("pri") {
            let mut chars = priority.chars();
            if let (Some(priority), None) = (chars.next(), chars.next())
                && priority.is_ascii_uppercase()
            {
                self.priority = Some(priority);
            }
        }
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

        let today = Local::now().date_naive();
        // FIXME: make the number of days configurable
        let due_start_to_be_urgent = today.checked_sub_days(Days::new(2)).unwrap_or(today);
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

        let one_month_ago = today.checked_sub_months(Months::new(1)).unwrap_or(today);

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
    use crate::{
        error::TaskError,
        tasks::strategies::{arbitrary_line, arbitrary_text, valid_task},
    };
    use proptest::prelude::*;

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
        assert_eq!(task.tags.get("pri").map(String::as_str), Some("A"));
    }

    #[test]
    fn it_round_trips_priority_through_completion_and_undo() {
        let mut task = Task {
            subject: "Some task to do".to_string(),
            priority: Some('B'),
            ..Task::default()
        };

        task.complete();
        let completed_line = task.to_string();
        let mut parsed = Task::from_str(1, &completed_line).unwrap();

        assert_eq!(parsed.priority, None);
        assert_eq!(parsed.tags.get("pri").map(String::as_str), Some("B"));

        parsed.undo();

        assert_eq!(parsed.priority, Some('B'));
        assert!(!parsed.tags.contains_key("pri"));
        assert!(!parsed.completed);
        assert_eq!(
            parsed.to_string(),
            format!("(B) {} Some task to do", Local::now().date_naive())
        );
    }

    #[test]
    fn it_rejects_an_empty_subject() {
        let err = Task::from_str(1, "").unwrap_err();

        assert!(matches!(err.current_context(), TaskError::EmptySubject));
    }

    #[test]
    fn it_rejects_an_invalid_priority() {
        let err = Task::from_str(1, "(a) a task").unwrap_err();

        assert!(matches!(
            err.current_context(),
            TaskError::InvalidPriority('a')
        ));
    }

    #[test]
    fn it_keeps_a_hashtag_in_the_subject_when_followed_by_another_token() {
        let task = Task::from_str(1, "task #next due:2024-06-01").unwrap();

        assert_eq!(task.subject, "task #next");
        assert_eq!(task.hashtags, vec!["next".to_string()]);
    }

    #[test]
    fn it_handles_short_malformed_prefixes_without_panicking() {
        let task = Task::from_str(1, "x").unwrap();
        assert_eq!(task.subject, "x");

        let task = Task::from_str(1, "x 2024-05-01").unwrap();
        assert_eq!(task.subject, "2024-05-01");

        let task = Task::from_str(1, "(").unwrap();
        assert_eq!(task.subject, "(");
    }

    proptest! {
        #[test]
        fn arbitrary_text_never_panics(input in arbitrary_text()) {
            match Task::from_str(7, &input) {
                Ok(task) => {
                    prop_assert_eq!(task.id, 7);
                    prop_assert!(!task.subject.is_empty());
                }
                Err(_) => {}
            }
        }

        #[test]
        fn arbitrary_lines_never_panics(line in arbitrary_line()) {
            match Task::from_str(7, &line) {
                Ok(task) => prop_assert!(!task.subject.is_empty()),
                Err(_) => {}
            }
        }

        #[test]
        fn canonical_task_round_trips_through_display_and_parse(task in valid_task()) {
            let line = task.to_string();
            let result = Task::from_str(1, &line);
            prop_assert!(result.is_ok(), "failed to parse generated line: {line}");
            let parsed = result.unwrap();

            prop_assert_eq!(parsed.subject, task.subject);
            prop_assert_eq!(parsed.priority, task.priority);
            prop_assert_eq!(parsed.created_at, task.created_at);
            prop_assert_eq!(parsed.due_date, task.due_date);
            prop_assert_eq!(parsed.contexts, task.contexts);
            prop_assert_eq!(parsed.projects, task.projects);
            prop_assert_eq!(parsed.hashtags, task.hashtags);
            prop_assert_eq!(parsed.tags, task.tags);
        }

        #[test]
        fn completion_and_undo_restore_the_original_priority(task in valid_task()) {
            let original_priority = task.priority;
            let original_subject = task.subject.clone();
            let mut task = task;

            task.complete();
            prop_assert!(task.completed);

            let line = task.to_string();
            let result = Task::from_str(1, &line);
            prop_assert!(result.is_ok(), "failed to parse completed task: {line}");
            let mut parsed = result.unwrap();

            parsed.undo();

            prop_assert_eq!(parsed.subject, original_subject);
            prop_assert_eq!(parsed.priority, original_priority);
            prop_assert!(!parsed.completed);
            prop_assert!(!parsed.tags.contains_key("pri"));
        }
    }
}
