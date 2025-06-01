use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, ErrorKind};
use serde_json;

#[derive(Serialize, Deserialize)c]
pub struct Task {
    pub text: String,
    pub done: bool,
}

pub fn load_tasks(filename: &str) -> io::Result<Vec<Task>> {
    match fs::read_to_string(filename) {
        Ok(data) => {
            let tasks: Vec<Task> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
            Ok(tasks)
        }
        Err(ref e) if e.kind() == ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e),
    }
}

pub fn save_tasks(filename: &str, tasks: &[Task]) -> io::Result<()> {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(filename, data)
}