mod datetime;
mod recurrence;
mod task;

use anyhow::{anyhow, Result};
use home::home_dir;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use crate::task::{Task, TASK_START};

fn get_md_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
    for entry in path.read_dir()? {
        let path = entry?.path();
        if path.is_dir() {
            result.append(&mut get_md_files(path)?);
        } else if path.extension().map_or(false, |ext| ext == "md") {
            result.push(path);
        }
    }
    Ok(result)
}

fn normalize_file(path: &PathBuf) -> Result<()> {
    let file = File::open(&path)?;
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.starts_with(TASK_START) {
            let task = Task::from_string(&line, path.clone(), line_number)?;
            println!("{:?}", task);
            lines.push(task.to_string());
        } else {
            lines.push(line);
        }
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for line in lines {
        writeln!(writer, "{}", line)?;
    }
    writer.flush()?;

    return Ok(());
}

fn main() -> Result<()> {
    // get path
    let home_path = if let Some(home_path) = home_dir() {
        home_path
    } else {
        return Err(anyhow!("Can't get home dir"));
    };
    let path = home_path.join("documents/notes");

    // get all files
    let paths = get_md_files(path)?;

    // show conten
    let _ = paths.iter().try_for_each(|path| normalize_file(path));

    return Ok({});
}
