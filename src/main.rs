mod task;
mod storage;
mod cli;

use task::Task;
use storage::{save_tasks, load_tasks};
use cli::{get_user_input, list_tasks, mark_task_as_complete, delete_task, edit_task};

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("-- To-Do List Menu --");
        println!("1. Add a new task");
        println!("2. List all tasks");
        println!("3. Mark a task as complete");
        println!("4. Delete a task");
        println!("5. Edit a task");
        println!("6. Exit");

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
            "4" => {
                delete_task(&mut tasks);
                save_tasks(&tasks);
            }
            "5" => {
                edit_task(&mut tasks);
                save_tasks(&tasks);
            }
            "6" => break,
            _ => println!("Invalid choice, please try again. \n"),
        }
    }
}