use std::io;
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
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("-- To-Do List Menu --");
        println!("1. Add a new task");
        println!("2. List all tasks");
        println!("3. Exit");

        let choice = get_user_input("Enter your option: ");

        match choice.trim() {
            "1" => {
                let description = get_user_input("Enter a description: ");
                tasks.push(Task::new(description));
                println!("Task added!\n");
            }
            "2" => list_tasks(&tasks),
            "3" => break,
            _ => println!("Invalid choice, please try again. \n"),
        }
    }
}

fn list_tasks(tasks: &Vec<Task>) {
    println!("\n");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✓" } else { " " };
        println!("{}: [{}] {}", index + 1, status, task.description);
    }
    println!("\n");
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
