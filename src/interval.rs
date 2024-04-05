use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interval {
    pub uuid: String,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}
