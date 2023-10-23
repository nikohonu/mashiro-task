use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct Interval {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub project: String,
    pub schedule: Option<NaiveDateTime>,
    pub recurrence_type: Option<String>,
    pub recurrence_unit: Option<String>,
    pub recurrence: Option<i64>,
    pub completions: Vec<NaiveDateTime>,
    pub intervals: Vec<Interval>,
}
