use anyhow::Result;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Debug)]
pub struct DateTime {
    datetime: NaiveDateTime,
}

fn get_end_of_day() -> NaiveTime {
    NaiveTime::from_hms_opt(23, 59, 59).unwrap()
}

impl DateTime {
    fn from_datetime(datetime: NaiveDateTime) -> Self {
        Self { datetime }
    }

    fn from_date(date: NaiveDate) -> Self {
        Self::from_datetime(NaiveDateTime::new(date, get_end_of_day()))
    }

    pub fn from_string(string: &str) -> Result<Self> {
        match string {
            "today" => Ok(Self::from_date(Local::now().date_naive())),
            _ => Ok(string
                .parse::<NaiveDate>()
                .map(Self::from_date)
                .or_else(|_| string.parse::<NaiveDateTime>().map(Self::from_datetime))?),
        }
    }

    pub fn to_string(&self) -> String {
        if self.datetime.time() == get_end_of_day() {
            self.datetime.date().to_string()
        } else {
            self.datetime.to_string()
        }
    }
}
