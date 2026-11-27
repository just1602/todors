use std::collections::HashMap;

use chrono::{Duration, Local, NaiveDate};
use proptest::prelude::*;

use super::task::Task;

#[derive(Debug, PartialEq)]
pub(crate) struct TaskSnapshot {
    pub(crate) subject: String,
    pub(crate) priority: Option<char>,
    pub(crate) created_at: Option<NaiveDate>,
    pub(crate) completed_at: Option<NaiveDate>,
    pub(crate) completed: bool,
    pub(crate) due_date: Option<NaiveDate>,
    pub(crate) contexts: Vec<String>,
    pub(crate) projects: Vec<String>,
    pub(crate) hashtags: Vec<String>,
    pub(crate) tags: HashMap<String, String>,
}

impl TaskSnapshot {
    pub(crate) fn from_task(task: &Task) -> Self {
        Self {
            subject: task.subject.clone(),
            priority: task.priority,
            created_at: task.created_at,
            completed_at: task.completed_at,
            completed: task.completed,
            due_date: task.due_date,
            contexts: task.contexts.clone(),
            projects: task.projects.clone(),
            hashtags: task.hashtags.clone(),
            tags: task.tags.clone(),
        }
    }
}

pub(crate) fn safe_word() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(('a'..='z').collect::<Vec<_>>()), 1..8)
        .prop_map(|chars| chars.into_iter().collect())
}

fn valid_date() -> impl Strategy<Value = NaiveDate> {
    prop::sample::select(
        ["2024-01-02", "2024-05-01", "2025-12-31", "2026-02-28"]
            .map(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap())
            .to_vec(),
    )
}

fn uppercase_priority() -> impl Strategy<Value = Option<char>> {
    prop::option::of(prop::sample::select(
        (b'A'..=b'Z').map(char::from).collect::<Vec<_>>(),
    ))
}

fn literal(value: &'static str) -> impl Strategy<Value = String> {
    Just(value).prop_map(str::to_owned)
}

fn tag_key() -> impl Strategy<Value = String> {
    safe_word().prop_map(|mut word| {
        if word == "due" {
            word.push('x');
        }
        word
    })
}

pub(crate) fn valid_task() -> impl Strategy<Value = Task> {
    (
        prop::collection::vec(safe_word(), 1..5),
        prop::collection::vec(safe_word(), 0..3),
        prop::collection::vec(safe_word(), 0..3),
        prop::collection::vec(safe_word(), 0..3),
        prop::collection::vec((tag_key(), safe_word()), 0..3),
        uppercase_priority(),
        prop::option::of(valid_date()),
        prop::option::of(valid_date()),
    )
        .prop_map(
            |(words, contexts, projects, hashtags, tags, priority, created_at, due_date)| {
                let mut parts = vec!["task".to_owned()];
                parts.extend(words);
                parts.extend(contexts.iter().map(|context| format!("@{context}")));
                parts.extend(projects.iter().map(|project| format!("+{project}")));
                parts.extend(hashtags.iter().map(|hashtag| format!("#{hashtag}")));

                Task {
                    id: 1,
                    subject: parts.join(" "),
                    priority,
                    created_at,
                    completed_at: None,
                    completed: false,
                    due_date,
                    contexts,
                    projects,
                    hashtags,
                    tags: tags.into_iter().collect::<HashMap<_, _>>(),
                }
            },
        )
}

pub(crate) fn arbitrary_line() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            literal("x"),
            literal("x 2024-05-01"),
            literal("(A)"),
            literal("(a)"),
            literal("2024-05-01"),
            literal("2024-13-45"),
            literal("+project"),
            literal("@home"),
            literal("#next"),
            literal("due:2024-06-01"),
            literal("team:devops"),
            literal("key:"),
            literal(":"),
            literal("@"),
            literal("+"),
            literal("#"),
            literal("("),
            literal("-"),
            literal(","),
            literal("1"),
            literal("1-3"),
            literal("é"),
            literal("🚀"),
            literal(" "),
            safe_word(),
        ],
        0..12,
    )
    .prop_map(|tokens| tokens.join(" "))
}

pub(crate) fn arbitrary_text() -> impl Strategy<Value = String> {
    prop_oneof![any::<String>(), arbitrary_line()]
}

fn relative_date() -> impl Strategy<Value = NaiveDate> {
    prop::sample::select(vec![-400i64, -10, 3, 400]).prop_map(|days| {
        Local::now()
            .date_naive()
            .checked_add_signed(Duration::days(days))
            .unwrap()
    })
}

pub(crate) fn urgency_task() -> impl Strategy<Value = Task> {
    (
        0usize..50,
        uppercase_priority(),
        prop::option::of(relative_date()),
        prop::option::of(relative_date()),
        prop::collection::vec(
            prop::sample::select(vec!["next", "inbox", "backlog", "later"]),
            0..4,
        ),
        prop::collection::vec(safe_word(), 0..2),
    )
        .prop_map(
            |(id, priority, created_at, due_date, hashtags, projects)| Task {
                id,
                subject: format!("task {id}"),
                priority,
                created_at,
                completed_at: None,
                completed: false,
                due_date,
                contexts: Vec::new(),
                projects,
                hashtags: hashtags.into_iter().map(str::to_owned).collect(),
                tags: HashMap::new(),
            },
        )
}
