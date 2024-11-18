use crate::task::{Task, TASK_START};
use anyhow::{anyhow, Result};
use comfy_table::{modifiers::UTF8_SOLID_INNER_BORDERS, presets::UTF8_FULL, Table};
use home::home_dir;
use rand::{distributions::Alphanumeric, Rng};
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    fn get_md_files(path: PathBuf) -> Result<Vec<PathBuf>> {
        let mut result: Vec<PathBuf> = Vec::new();
        for entry in path.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                result.append(&mut Self::get_md_files(path)?);
            } else if path.extension().map_or(false, |ext| ext == "md") {
                result.push(path);
            }
        }
        Ok(result)
    }

    fn load_tasks(path: &PathBuf) -> Result<Vec<Task>> {
        let file = File::open(&path)?;
        let reader = BufReader::new(&file);
        let mut tasks: Vec<Task> = Vec::new();

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;
            if line.starts_with(TASK_START) {
                let task = Task::from_string(&line, path.clone(), line_number)?;
                tasks.push(task);
            }
        }

        return Ok(tasks);
    }

    fn generate_ids(self: &mut Self) {
        let ids: Vec<_> = self
            .tasks
            .iter()
            .filter_map(|task| task.id.clone())
            .collect();

        for task in self.tasks.iter_mut() {
            if task.id.is_some() {
                continue;
            }

            loop {
                let id = Self::generate_id();
                if ids.contains(&&id) {
                    continue;
                }
                task.id = Some(id);
                task.is_updated = true;
                break;
            }
        }
    }

    fn get_path() -> Result<PathBuf> {
        let home_path = if let Some(home_path) = home_dir() {
            home_path
        } else {
            return Err(anyhow!("Can't get home dir"));
        };
        Ok(home_path.join("local-git/notes"))
    }

    pub fn new() -> Result<Tasks> {
        let paths = Self::get_md_files(Tasks::get_path()?)?;
        let tasks: Vec<Task> = paths
            .iter()
            .map(|path| Self::load_tasks(path))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        let mut tasks = Tasks { tasks };
        tasks.generate_ids();
        return Ok(tasks);
    }

    fn generate_id() -> String {
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();
        random_string
    }

    pub fn print(&self) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS)
            .set_header(vec![
                "id", "priority", "name", "recur", "schedule", "project",
            ]);
        let empty = String::new();
        for task in &self.tasks {
            table.add_row(vec![
                task.id.clone().unwrap_or(empty.clone()),
                task.priority
                    .clone()
                    .map_or(empty.clone(), |recur| recur.to_string()),
                task.name.clone(),
                task.recurrence
                    .clone()
                    .map_or(empty.clone(), |recur| recur.to_string()),
                task.schedule
                    .clone()
                    .map_or(empty.clone(), |schedule| schedule.to_string()),
                task.get_project(),
            ]);
        }
        println!("{table}");
    }

    pub fn filter(&mut self, predicate: fn(&Task) -> bool) {
        self.tasks = self.tasks.clone().into_iter().filter(predicate).collect();
    }

    fn sub_compare<T>(a: &Option<T>, b: &Option<T>) -> Option<Ordering>
    where
        T: PartialEq + PartialOrd,
    {
        if let (Some(some_a), Some(some_b)) = (&a, &b) {
            // if task priority same, we need to use other ordering method
            if some_a != some_b {
                return Some(some_a.partial_cmp(some_b).unwrap());
            }
        }
        // if one of task don't have priority
        if a.is_some() {
            return Some(Ordering::Less);
        } else if b.is_some() {
            return Some(Ordering::Greater);
        }
        None
    }

    pub fn compare_task(task_a: &Task, task_b: &Task, priority_first: bool) -> Ordering {
        let (first, second) = if priority_first {
            (
                Self::sub_compare(&task_a.priority, &task_b.priority),
                Self::sub_compare(&task_a.schedule, &task_b.schedule),
            )
        } else {
            (
                Self::sub_compare(&task_a.schedule, &task_b.schedule),
                Self::sub_compare(&task_a.priority, &task_b.priority),
            )
        };
        first.or(second).unwrap_or(Ordering::Equal)
    }

    pub fn sort(&mut self, priority_first: bool) {
        self.tasks
            .sort_by(|a, b| Tasks::compare_task(a, b, priority_first));
    }

    pub fn reduce(&mut self, number: usize) {
        self.tasks = self.tasks.clone().into_iter().take(number).collect();
    }

    pub fn done(&mut self, task_id: &str) {
        for task in &mut self.tasks {
            if let Some(current_task_id) = &task.id {
                if current_task_id == task_id {
                    task.done();
                }
            }
        }
    }
}

impl Drop for Tasks {
    fn drop(&mut self) {
        for task in &self.tasks {
            let _ = task.update_file();
        }
    }
}
