use std::iter::Peekable;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};

use crate::datetime::DateTime;
use crate::recurrence::Recurrence;

pub const TASK_START: &str = "- [ ] ";

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<String>,
    pub name: String,
    pub schedule: Option<DateTime>,
    pub recurrence: Option<Recurrence>,
    pub priority: Option<char>,
    file_path: PathBuf,
    line: usize,
    done: bool,
    pub is_updated: bool,
}

fn parse_name<'a, I>(elements: Peekable<I>) -> (String, Peekable<I>)
where
    I: Iterator<Item = &'a str> + Clone,
{
    let mut elements = elements.clone();
    let mut name: Vec<&str> = Vec::new();
    while let Some(&element) = elements.peek() {
        if ["schedule:", "recur:", "id:"]
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
        let mut schedule = None;
        let mut recurrence = None;
        let mut id = None;
        let mut priority = None;
        for element in elements {
            let (field, value) = element.split_once(':').unwrap_or((element, ""));
            match field {
                "schedule" => schedule = Some(DateTime::from_string(value)?),
                "recur" => recurrence = Some(Recurrence::from_string(value)?),
                "priority" => priority = Some(value.to_string().remove(0)),
                "id" => id = Some(value.to_string()),
                _ => {}
            }
        }
        Ok(Task {
            id,
            priority,
            name,
            schedule,
            recurrence,
            file_path,
            line,
            done: false,
            is_updated: false,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "- [{}] {}{}{}{}{}",
            if self.done {
                "x".to_string()
            } else {
                " ".to_string()
            },
            self.name,
            self.schedule
                .as_ref()
                .map_or(String::new(), |schedule| format!(
                    " schedule:{}",
                    schedule.to_string()
                )),
            self.recurrence
                .as_ref()
                .map_or(String::new(), |recurrence| format!(
                    " recur:{}",
                    recurrence.to_string()
                )),
            self.priority
                .as_ref()
                .map_or(String::new(), |priority| format!(
                    " priority:{}",
                    priority.to_string()
                )),
            self.id
                .as_ref()
                .map_or(String::new(), |id| format!(" id:{}", id.to_string())),
        )
    }

    pub fn update_file(&self) -> Result<()> {
        // println!("{:?}", self);
        if !self.is_updated {
            return Ok(());
        }
        let mut lines: Vec<String> = Vec::new();
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(&file);

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;
            if line_number == self.line {
                lines.push(self.to_string())
            } else {
                lines.push(line);
            }
        }

        let file = File::create(&self.file_path)?;
        let mut writer = BufWriter::new(file);
        for line in lines {
            writeln!(writer, "{}", line)?;
        }
        writer.flush()?;

        Ok(())
    }

    pub fn done(&mut self) {
        if let (Some(schedule), Some(recurrence)) = (&self.schedule, &self.recurrence) {
            self.schedule = Some(schedule.done(&recurrence));
        } else {
            self.done = true;
        }

        // let _ = self.update_file();
        self.is_updated = true;

        // println!("{:?}", self);
    }

    pub fn get_project(&self) -> String {
        return self.file_path.file_stem().map_or(String::new(), |stem| {
            stem.to_str().unwrap_or("").to_string()
        });
    }
}
