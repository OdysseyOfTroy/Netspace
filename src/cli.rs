use std::io;
use crate::task::Task;

pub fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn list_tasks(tasks: &Vec<Task>) {
    println!("\n-- Your Tasks --");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✓" } else { " " };
        println!("{}: [{}] {}", index + 1, status, task.description);
    }
    println!("");
}

pub fn mark_task_as_complete(tasks: &mut Vec<Task>) {
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