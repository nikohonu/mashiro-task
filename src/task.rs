use std::iter::Peekable;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::datetime::DateTime;
use crate::recurrence::Recurrence;

pub const TASK_START: &str = "- [ ] ";

#[derive(Debug)]
pub struct Task {
    name: String,
    due: Option<DateTime>,
    recurrence: Option<Recurrence>,
    file_path: PathBuf,
    line: usize,
}

fn parse_name<'a, I>(elements: Peekable<I>) -> (String, Peekable<I>)
where
    I: Iterator<Item = &'a str> + Clone,
{
    let mut elements = elements.clone();
    let mut name: Vec<&str> = Vec::new();
    while let Some(&element) = elements.peek() {
        if ["due:", "recur:", "id:"]
            .iter()
            .any(|&field| element.starts_with(field))
        {
            break;
        }
        elements.next().map(|element| name.push(element));
    }
    (name.join(" "), elements)
}

impl Task {
    pub fn from_string(string: &str, file_path: PathBuf, line: usize) -> Result<Task> {
        if !string.starts_with(TASK_START) {
            return Err(anyhow!("It's not a task!"));
        }
        let elements = string[TASK_START.len()..].split_whitespace().peekable();
        let (name, elements) = parse_name(elements);
        let mut due = None;
        let mut recurrence = None;
        for element in elements {
            let (field, value) = element.split_once(':').unwrap_or((element, ""));
            match field {
                "due" => due = Some(DateTime::from_string(value)?),
                "recur" => recurrence = Some(Recurrence::from_string(value)?),
                _ => {}
            }
        }
        Ok(Task {
            name,
            due,
            recurrence,
            file_path,
            line,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "- [ ] {}{}{}",
            self.name,
            self.due
                .as_ref()
                .map_or(String::new(), |due| format!(" due:{}", due.to_string())),
            self.recurrence
                .as_ref()
                .map_or(String::new(), |recurrence| format!(
                    " recur:{}",
                    recurrence.to_string()
                ))
        )
    }
}
