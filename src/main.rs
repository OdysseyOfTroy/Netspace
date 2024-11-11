use serde::{Deserialize, Serialize};
use std::io::{self, BufReader};

use serde_json::{self, to_writer};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}
fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("-- To-Do List Menu --");
        println!("1. Add a new task");
        println!("2. List all tasks");
        println!("3. Mark a task as complete");
        println!("4. Exit");

        let choice = get_user_input("Enter your option: ");

        match choice.trim() {
            "1" => {
                let description = get_user_input("Enter a description: ");
                tasks.push(Task::new(description));
                save_tasks(&tasks);
                println!("Task added!\n");
            }
            "2" => list_tasks(&tasks),
            "3" => {
                mark_task_as_complete(&mut tasks);
                save_tasks(&tasks);
            }
            "4" => break,
            _ => println!("Invalid choice, please try again. \n"),
        }
    }
}

fn list_tasks(tasks: &Vec<Task>) {
    println!("\n-- Your Tasks --");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✓" } else { " " };
        println!("{}: [{}] {}", index + 1, status, task.description);
    }
    println!("");
}

fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create("tasks.json").expect("could not create file.");
    serde_json::to_writer(file, tasks).expect("could not write data to file.");
    println!("Tasks saved successfully.\n");
}

fn load_tasks() -> Vec<Task> {
    if let Ok(file) = File::open("tasks.json") {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn mark_task_as_complete(tasks: &mut Vec<Task>) {
    list_tasks(&tasks);
    let task_number = get_user_input("Enter the number of the task you wish to mark as completed");

    match task_number.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= tasks.len() => {
            tasks[num - 1].completed = true;
            println!("Task {} marked as complete!\n", num);
        }
        _ => println!("Invalid task number.\n"),
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
