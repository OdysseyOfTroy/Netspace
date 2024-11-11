fn main() {
    let mut  tasks: Vec<Task> = Vec::new();

    tasks.push(Task::new("Learn rust basics".to_string()));

    list_tasks(&tasks);
}
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: true,
        }
    }
}

fn list_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✓" } else { " " };
        println!("{}: [{}] {}", index + 1, status, task.description);
    }
}