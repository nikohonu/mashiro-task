use chrono::NaiveDateTime;

use crate::task::Task;
#[derive(clap::Args, Debug)]
pub struct DoArgs {
    id: i64,
}

fn calc_new_schedule(
    schedule: NaiveDateTime,
    recurrence: i64,
    recurrence_unit: &str,
) -> Result<NaiveDateTime, &'static str> {
    match recurrence_unit {
        "d" => {
            return Ok(schedule + chrono::Duration::days(recurrence));
        }
        _ => return Err("This kind of recurrence unit don't implement yet"),
    }
}

impl DoArgs {
    pub fn run(&self) {
        let tasks = Task::all();
        let task = Task::by_id(&tasks, self.id);
        if task.is_none() {
            println!("There is no task with id={}", self.id);
            return;
        }
        let mut task = task.unwrap();
        println!("{:?}", task);

        let now = chrono::Local::now().naive_local();
        task.schedule = match task.recurrence_type.as_str() {
            "+" => {
                calc_new_schedule(task.schedule, task.recurrence, &task.recurrence_unit).unwrap()
            }
            "++" => {
                let mut new_schedule = task.schedule;
                loop {
                    new_schedule =
                        calc_new_schedule(new_schedule, task.recurrence, &task.recurrence_unit)
                            .unwrap();
                    if new_schedule > now {
                        break new_schedule;
                    }
                }
            }
            ".+" => {
                if task.recurrence_unit == "h" {
                    calc_new_schedule(now, task.recurrence, &task.recurrence_unit).unwrap()
                } else {
                    calc_new_schedule(
                        NaiveDateTime::new(now.date(), task.schedule.time()),
                        task.recurrence,
                        &task.recurrence_unit,
                    )
                    .unwrap()
                }
            }
            "c" => calc_new_schedule(now, task.recurrence, &task.recurrence_unit).unwrap(),
            _ => panic!(),
        };
        task.now_datetime = None;
        println!("New schedule: {}", task.schedule);
        Task::update_one(task);
    }
}
