use crate::paths::get_tasks_path;
use chrono::NaiveDateTime;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interval {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub uuid: String,
    pub name: String,
    pub project: String,
    pub schedule: Option<NaiveDateTime>,
    pub deadline: Option<NaiveDateTime>,
    pub recurrence_type: Option<String>,
    pub recurrence_unit: Option<String>,
    pub recurrence: Option<i64>,
    pub required: bool,
    pub required_task: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub completions: Vec<NaiveDateTime>,
    #[serde(skip_serializing, skip_deserializing)]
    pub intervals: Vec<Interval>,
}

impl Task {
    pub fn get_tasks() -> Vec<Task> {
        let tasks_path = get_tasks_path();
        let file = std::fs::File::open(tasks_path).expect("Can't open file");
        let mut reader = csv::Reader::from_reader(file);
        reader
            .deserialize::<Task>()
            .map(|x| x.expect("Can't read tasks files"))
            .collect()
    }
    pub fn append(task: &Task) {
        let tasks_path = get_tasks_path();
        let is_header = !tasks_path.exists();
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(tasks_path)
            .expect("Can't open file");
        let mut wrt = WriterBuilder::new()
            .has_headers(is_header)
            .from_writer(file);
        wrt.serialize(task).expect("I can't write new record.");
        wrt.flush().expect("I cant't write in file.");
    }
}
