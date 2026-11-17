use std::collections::HashMap;
use std::str::FromStr;

use chrono::NaiveDate;

use crate::error::{Report, TaskError, TaskResult, bail, report};

#[derive(Debug)]
pub struct TaskQuery {
    pub indexes: Vec<usize>,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub hashtags: Vec<String>,
    pub due_date: Option<NaiveDate>,
    pub tags: HashMap<String, String>,
    pub subject: String,
}

fn query_slice(source: &str, start: usize, end: usize) -> TaskResult<&str> {
    source.get(start..end).ok_or_else(|| {
        report!(TaskError::ParseQuery {
            detail: "invalid byte range in query".to_owned(),
        })
    })
}

fn append_query_slice(
    subject: &mut String,
    source: &str,
    start: usize,
    end: usize,
) -> TaskResult<()> {
    subject.push_str(query_slice(source, start, end)?);
    Ok(())
}

impl TaskQuery {
    pub fn from_string_vec(v: &[String]) -> TaskResult<TaskQuery> {
        let query = v.join(" ");

        query.parse()
    }
}

fn parse_index_range(s: &str, start: usize, dash: usize, end: usize) -> TaskResult<Vec<usize>> {
    let lhs_text = query_slice(s, start, dash)?;
    let Ok(lhs) = lhs_text.parse::<usize>() else {
        bail!(TaskError::ParseQuery {
            detail: format!("invalid index range start: {lhs_text}"),
        });
    };

    let rhs_text = query_slice(s, dash + 1, end)?;
    let Ok(rhs) = rhs_text.parse::<usize>() else {
        bail!(TaskError::ParseQuery {
            detail: format!("invalid index range end: {rhs_text}"),
        });
    };

    Ok((lhs..=rhs).collect())
}

impl FromStr for TaskQuery {
    type Err = Report<TaskError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum State {
            Init,
            Project(usize),
            Context(usize),
            HashTag(usize),
            Index(usize),
            Range(usize, usize),
            TagBegin(usize),
            TagEnd(usize, usize),
        }

        let mut state = State::Init;
        let mut subject = String::new();
        let mut indexes = Vec::new();
        let mut projects = Vec::new();
        let mut contexts = Vec::new();
        let mut hashtags = Vec::new();
        let mut tags = HashMap::new();
        let mut due_date = None;

        for (i, c) in s.char_indices() {
            let char_end = i + c.len_utf8();
            let new_state = match (c, state) {
                ('@', State::Init) => State::Context(i),
                ('+', State::Init) => State::Project(i),
                ('#', State::Init) => State::HashTag(i),
                ('0'..='9', State::Init) => State::Index(i),
                ('A'..='z', State::Init) => State::TagBegin(i),
                (':', State::TagBegin(j)) => State::TagEnd(j, i),
                (' ', State::TagBegin(j)) => {
                    append_query_slice(&mut subject, s, j, char_end)?;
                    State::Init
                }
                (' ', State::Project(j)) => {
                    if i > j + 1 {
                        projects.push(query_slice(s, j + 1, i)?.to_string());
                    }
                    State::Init
                }
                (' ', State::Context(j)) => {
                    if i > j + 1 {
                        contexts.push(query_slice(s, j + 1, i)?.to_string());
                    }
                    State::Init
                }
                (' ', State::HashTag(j)) => {
                    if i > j + 1 {
                        hashtags.push(query_slice(s, j + 1, i)?.to_string());
                    }
                    State::Init
                }
                ('-', State::Index(j)) => State::Range(j, i),
                (',' | ' ', State::Index(j)) => {
                    let text = query_slice(s, j, i)?;
                    let idx = text.parse::<usize>().map_err(|_| {
                        report!(TaskError::ParseQuery {
                            detail: format!("invalid index: {text}"),
                        })
                    })?;
                    indexes.push(idx);
                    State::Init
                }
                (',' | ' ', State::Range(j, k)) => {
                    let mut range = parse_index_range(s, j, k, i)?;
                    indexes.append(&mut range);
                    State::Init
                }
                (' ', State::TagEnd(j, k)) => {
                    if i > k + 1 {
                        let tag = query_slice(s, j, k)?;
                        let value = query_slice(s, k + 1, i)?;
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

            if new_state == State::Init && state == State::Init {
                subject.push(c);
            }

            state = new_state;
        }

        match state {
            State::TagEnd(j, k) if s.len() > k + 1 => {
                let tag = query_slice(s, j, k)?;
                let value = query_slice(s, k + 1, s.len())?;
                if tag == "due" {
                    due_date = value.parse::<NaiveDate>().ok();
                } else {
                    tags.insert(tag.to_string(), value.to_string());
                }
            }
            State::TagBegin(j) => {
                append_query_slice(&mut subject, s, j, s.len())?;
            }
            State::Project(j) => {
                if s.len() > j + 1 {
                    projects.push(query_slice(s, j + 1, s.len())?.to_string());
                }
            }
            State::Context(j) => {
                if s.len() > j + 1 {
                    contexts.push(query_slice(s, j + 1, s.len())?.to_string());
                }
            }
            State::HashTag(j) => {
                if s.len() > j + 1 {
                    hashtags.push(query_slice(s, j + 1, s.len())?.to_string());
                }
            }
            State::Range(j, k) => {
                let mut range = parse_index_range(s, j, k, s.len())?;
                indexes.append(&mut range);
            }
            State::Index(j) => {
                let text = query_slice(s, j, s.len())?;
                let idx = text.parse::<usize>().map_err(|_| {
                    report!(TaskError::ParseQuery {
                        detail: format!("invalid index: {text}"),
                    })
                })?;
                indexes.push(idx);
            }
            _ => {}
        }

        Ok(TaskQuery {
            indexes,
            projects,
            contexts,
            hashtags,
            due_date,
            tags,
            subject: subject.trim_end().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::TaskQuery;
    use crate::error::TaskError;
    use chrono::NaiveDate;
    use std::collections::HashMap;

    #[test]
    fn it_parse_subject() {
        let query = "test test".parse::<TaskQuery>().unwrap();

        assert_eq!(query.subject, "test test")
    }

    #[test]
    fn it_parse_project() {
        let query = "+test test something".parse::<TaskQuery>().unwrap();

        assert_eq!(query.subject, "test something");
        assert_eq!(query.projects, vec!["test"])
    }

    #[test]
    fn it_parse_context() {
        let query = "test @home".parse::<TaskQuery>().unwrap();

        assert_eq!(query.subject, "test");
        assert_eq!(query.contexts, vec!["home"])
    }

    #[test]
    fn it_parse_hastags() {
        let query = "test #home".parse::<TaskQuery>().unwrap();

        assert_eq!(query.subject, "test");
        assert_eq!(query.hashtags, vec!["home"])
    }

    #[test]
    fn it_parse_due_date() {
        let query = "test due:2024-08-01".parse::<TaskQuery>().unwrap();

        assert_eq!(query.subject, "test");
        assert_eq!(
            query.due_date,
            Some(NaiveDate::from_ymd_opt(2024, 8, 1).unwrap())
        );
    }

    #[test]
    fn it_parse_tags() {
        let query = "test team:sre".parse::<TaskQuery>().unwrap();

        assert_eq!(query.subject, "test");
        assert_eq!(
            query.tags,
            HashMap::from([("team".to_string(), "sre".to_string())])
        );
    }

    #[test]
    fn it_parse_index() {
        let query = "1".parse::<TaskQuery>().unwrap();

        assert_eq!(query.indexes, vec![1]);
    }

    #[test]
    fn it_parse_long_index() {
        let query = "109234".parse::<TaskQuery>().unwrap();

        assert_eq!(query.indexes, vec![109234]);
    }

    #[test]
    fn it_parse_index_enumeration() {
        // TODO: add fuzzing for those parsers
        let query = "1,12".parse::<TaskQuery>().unwrap();

        assert_eq!(query.indexes, vec![1, 12]);
    }

    #[test]
    fn it_parse_index_range() {
        let query = "2-5".parse::<TaskQuery>().unwrap();

        assert_eq!(query.indexes, vec![2, 3, 4, 5]);
    }

    #[test]
    fn it_parse_index_enumeration_and_range() {
        let query = "2-5, 9".parse::<TaskQuery>().unwrap();

        assert_eq!(query.subject, "");
        assert_eq!(query.indexes, vec![2, 3, 4, 5, 9]);
    }

    #[test]
    fn it_can_parse_directly_from_string_vec() {
        let clap_query = vec!["test".to_string(), "team:sre".to_string()];
        let query = TaskQuery::from_string_vec(&clap_query).unwrap();

        assert_eq!(query.subject, "test");
        assert_eq!(
            query.tags,
            HashMap::from([("team".to_string(), "sre".to_string())])
        );
    }

    #[test]
    fn it_reports_an_invalid_index_range() {
        let err = "2-".parse::<TaskQuery>().unwrap_err();

        assert!(matches!(
            err.current_context(),
            TaskError::ParseQuery { .. }
        ));

        let message = format!("{err}");

        assert!(message.contains("failed to parse query"), "{message}");
        assert!(message.contains("invalid index range end"), "{message}");
    }
}
