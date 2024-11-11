use std::fs::File;
use std::io::BufReader;
use serde_json;
use crate::task::Task;

pub fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create("tasks.json").expect("could not create file.");
    serde_json::to_writer(file, tasks).expect("could not write data to file.");
    println!("Tasks saved successfully.\n");
}

pub fn load_tasks() -> Vec<Task> {
    if let Ok(file) = File::open("tasks.json") {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}