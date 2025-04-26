use crate::tasks::error::TaskError;
use std::collections::HashMap;
use std::str::FromStr;

use chrono::NaiveDate;

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

impl TaskQuery {
    pub fn from_string_vec(v: &[String]) -> Result<TaskQuery, TaskError> {
        let query = v.join(" ");

        query.parse()
    }
}

impl FromStr for TaskQuery {
    type Err = TaskError;

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
        let mut subject = Vec::new();
        let mut indexes = Vec::new();
        let mut projects = Vec::new();
        let mut contexts = Vec::new();
        let mut hashtags = Vec::new();
        let mut tags = HashMap::new();
        let mut due_date = None;

        for (i, c) in s.chars().enumerate() {
            let new_state = match (c, state) {
                ('@', State::Init) => State::Context(i),
                ('+', State::Init) => State::Project(i),
                ('#', State::Init) => State::HashTag(i),
                ('0'..='9', State::Init) => State::Index(i),
                ('A'..='z', State::Init) => State::TagBegin(i),
                (':', State::TagBegin(j)) => State::TagEnd(j, i),
                (' ', State::TagBegin(j)) => {
                    subject.extend(&s.as_bytes()[j..i + 1]);

                    State::Init
                }
                (' ', State::Project(j)) => {
                    if i - j > 1 {
                        projects.push(s[j + 1..i].to_string());
                    }
                    State::Init
                }
                (' ', State::Context(j)) => {
                    if i - j > 1 {
                        contexts.push(s[j + 1..i].to_string());
                    }
                    State::Init
                }
                (' ', State::HashTag(j)) => {
                    if i - j > 1 {
                        hashtags.push(s[j + 1..i].to_string());
                    }
                    State::Init
                }
                ('-', State::Index(j)) => State::Range(j, i),
                // FIXME: test this correctly to see if all the cases works
                (',' | ' ', State::Index(j)) => {
                    if let Ok(idx) = s[j..i].parse::<usize>() {
                        indexes.push(idx)
                    }
                    State::Init
                }
                (',' | ' ', State::Range(j, k)) => {
                    // FIXME: handle error correctly here
                    let lhs = s[j..k].parse::<usize>().unwrap();
                    let rhs = s[k + 1..i].parse::<usize>().unwrap();
                    let mut range: Vec<usize> = (lhs..=rhs).collect();
                    indexes.append(&mut range);
                    State::Init
                }
                (' ', State::TagEnd(j, k)) => {
                    if i - k > 1 {
                        match &s[j..k] {
                            "due" => {
                                due_date = s[k + 1..i].parse::<NaiveDate>().ok();
                            }
                            tag => {
                                tags.insert(tag.to_string(), s[k + 1..i].to_string());
                            }
                        }
                    }
                    State::Init
                }

                _ => state,
            };

            if new_state == State::Init && state == State::Init {
                subject.push(s.as_bytes()[i])
            }

            state = new_state
        }

        // parse current tag when we hit EOL
        match state {
            State::TagEnd(j, k) => match &s[j..k] {
                "due" => {
                    due_date = s[k + 1..].parse::<NaiveDate>().ok();
                }
                tag => {
                    tags.insert(tag.to_string(), s[k + 1..].to_string());
                }
            },
            State::TagBegin(j) => {
                subject.extend(&s.as_bytes()[j..]);
            }
            State::Project(j) => {
                projects.push(s[j + 1..].to_string());
            }
            State::Context(j) => {
                contexts.push(s[j + 1..].to_string());
            }
            State::HashTag(j) => {
                hashtags.push(s[j + 1..].to_string());
            }
            State::Range(j, k) => {
                let lhs = s[j..k].parse::<usize>().unwrap();
                let rhs = s[k + 1..].parse::<usize>().unwrap();
                let mut range: Vec<usize> = (lhs..=rhs).collect();
                indexes.append(&mut range);
            }
            State::Index(j) => {
                if let Ok(idx) = s[j..].parse::<usize>() {
                    indexes.push(idx)
                }
            }
            _ => {}
        }

        let subject = String::from_utf8(subject).unwrap().trim_end().to_string();

        Ok(TaskQuery {
            indexes,
            projects,
            contexts,
            hashtags,
            due_date,
            tags,
            subject,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::TaskQuery;
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
}
