use chrono::{Days, Local, Months, NaiveDateTime, TimeDelta};

use crate::recurrence::{Recurrence, Repeater, Unit};
use crate::task::Task;

#[derive(clap::Args, Debug)]
pub struct DoArgs {
    id: u64,
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

fn calc_new_schedule(schedule: NaiveDateTime, recurrence: Recurrence) -> NaiveDateTime {
    let now = Local::now().naive_local();
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

impl DoArgs {
    pub fn run(&self) {
        let tasks = Task::all();
        let task = Task::by_id(&tasks, self.id);
        if task.is_none() {
            panic!("There is no task with id = {}", self.id);
        }
        let mut task = task.unwrap();
        if task.recurrence.is_none() {
            Task::append(&task, true);
            Task::remove(self.id);
            println!(
                "Task \"{} {} - {}\" was archived",
                task.id, task.name, task.sphere
            );
            return;
        }
        let schedule = if let Some(schedule) = task.schedule {
            schedule
        } else {
            Local::now().naive_local()
        };
        task.schedule = Some(calc_new_schedule(
            schedule,
            task.recurrence.clone().unwrap(),
        ));
        println!(
            "New schedule for task {} {} - {}: {}",
            &task.id,
            &task.name,
            &task.sphere,
            task.schedule.unwrap()
        );
        Task::update_one(task);
    }
}
