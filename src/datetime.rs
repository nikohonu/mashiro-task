use crate::recurrence::{Recurrence, Repeater, Unit};
use anyhow::Result;
use chrono::{Days, Local, Months, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

#[derive(Debug, Clone)]
pub struct DateTime {
    datetime: NaiveDateTime,
}

fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}
fn get_day_start() -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
}

fn add_duration(datetime: NaiveDateTime, value: u64, unit: &Unit) -> NaiveDateTime {
    let datetime = datetime.clone();
    match unit {
        Unit::Hourly => datetime.checked_add_signed(TimeDelta::hours(value as i64)),
        Unit::Daily => datetime.checked_add_days(Days::new(value)),
        Unit::Weekly => datetime.checked_add_days(Days::new(value * 7)),
        Unit::Monthly => datetime.checked_add_months(Months::new(value as u32)),
        Unit::Yearly => datetime.checked_add_months(Months::new(value as u32 * 12)),
    }
    .unwrap()
}

fn calc_new_schedule(schedule: NaiveDateTime, recurrence: &Recurrence) -> NaiveDateTime {
    let now = get_now();
    match recurrence.repeater {
        Repeater::Absolute => add_duration(schedule, recurrence.value, &recurrence.unit),
        Repeater::Smart => {
            let mut schedule = schedule;
            loop {
                schedule = add_duration(schedule, recurrence.value, &recurrence.unit);
                if schedule > now {
                    break schedule;
                }
            }
        }
        Repeater::Relative => {
            let schedule = if recurrence.unit == Unit::Hourly {
                now
            } else {
                NaiveDateTime::new(now.date(), schedule.time())
            };
            add_duration(schedule, recurrence.value, &recurrence.unit)
        }
    }
}

impl DateTime {
    fn from_datetime(datetime: NaiveDateTime) -> Self {
        Self { datetime }
    }

    fn from_date(date: NaiveDate) -> Self {
        Self::from_datetime(NaiveDateTime::new(date, get_day_start()))
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
        if self.datetime.time() == get_day_start() {
            self.datetime.date().to_string()
        } else {
            self.datetime.to_string()
        }
    }

    pub fn done(&self, recurrence: &Recurrence) -> Self {
        Self {
            datetime: calc_new_schedule(self.datetime, recurrence),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.datetime <= get_now()
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.datetime == other.datetime
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.datetime.partial_cmp(&other.datetime)
    }
}
