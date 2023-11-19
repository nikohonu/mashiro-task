pub fn get_data_local_path() -> std::path::PathBuf {
    let data_local_dir = dirs::data_local_dir().expect("Can't get data_local_dir");
    let data_local_path = data_local_dir.join("mashiro-task");
    let _ = std::fs::create_dir_all(&data_local_path);
    data_local_path
}

pub fn get_tasks_path() -> std::path::PathBuf {
    get_data_local_path().join("tasks.csv")
}
