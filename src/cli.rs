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

fn get_task_index(tasks: &Vec<Task>, prompt: &str) -> Option<usize> {
    if tasks.is_empty() {
        println!("No tasks available.\n");
        return None;
    }

    let task_number = get_user_input(prompt);

    match task_number.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= tasks.len() => Some(num - 1),
        _ => {
            println!("Invalid task number.\n");
            None
        }
    }
}

pub fn mark_task_as_complete(tasks: &mut Vec<Task>) {
    list_tasks(&tasks);
    if let Some(index) = get_task_index(tasks, "Enter the number of the task you wish to mark as completed") {
        tasks[index].completed = true;
            println!("Task {} marked as complete!\n", index + 1);
    };
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    list_tasks(tasks);
    if let Some(index) = get_task_index(tasks, "Enter the number of the task you wish to delete: ") {
        tasks.remove(index);
        println!("Task {} deleted.\n", index + 1);
    };
}