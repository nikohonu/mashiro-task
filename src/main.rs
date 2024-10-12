mod task;

use anyhow::{Result};
use home::home_dir;
use std::{
    fs::{File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::task::Task;

fn get_md_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
    for dir_entry in path.read_dir()? {
        let path = dir_entry?.path();
        if path.is_dir() {
            result.append(&mut get_md_files(path)?);
            continue;
        }
        if let Some(extension) = path.extension() {
            if extension == "md" {
                result.push(path);
            }
        }
    }
    return Ok(result);
}

fn normalize_file(path: PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("- [ ] ") {
            let task = Task::from_string(&line)?;
            lines.push(task.to_string());
        } else {
            lines.push(line);
        }
    }
    for line in lines {
        println!("{}", line);
    }
    return Ok({});
}

fn main() -> Result<()> {
    // get path
    let home_path = home_dir().expect("Can't get home dir");
    let home_path = home_path.as_path();
    let path = home_path.join("documents/notes");
    // get all files
    let paths = get_md_files(path)?;
    // show conten
    for path in paths {
        _ = normalize_file(path);
        // println!("{}", fs::read_to_string(path)?);
    }
    return Ok({});
}
