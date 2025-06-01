    mod tasks;
    use std::env;
    use std::process;
    use tasks::{Task, load_tasks, save_tasks};

fn main() {

    
    let args: Vec<String> = env::args().collect();
    let filename = "tasks.json";

    let mut tasks = load_tasks(filename).unwrap_or_else(|err| {
        eprintln!("Error loading tasks: {}", err);
        process::exit(1);
    });

    match args.len() {
        1 => {
            println!("Usage: {} [add|list|complete] [task]", args[0]);
            process::exit(1);
        }
        2 => {
            if args[1] == "list" {
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "✓" } else { "✗" };
                    println!("{}: [{}] {}", i + 1, status, task.text);
                }
            } else {
                eprintln!("Invalid command. Use 'add', 'list', or 'complete'.");
                process::exit(1);
            }
        }
        3 => {
            if args[1] == "add" {
                let task_text = &args[2];
                tasks.push(Task {
                    text: task_text.clone(),
                    done: false,
                });
                save_tasks(filename, &tasks).unwrap_or_else(|err| {
                    eprintln!("Error saving tasks: {}", err);
                    process::exit(1);
                });
                println!("Added task: {}", task_text);
            } else if args[1] == "complete" {
                let index: usize = args[2].parse().unwrap_or_else(|_| {
                    eprintln!("Invalid task index.");
                    process::exit(1);
                });
                if index > 0 && index <= tasks.len() {
                    tasks[index - 1].done = true;
                    save_tasks(filename, &tasks).unwrap_or_else(|err| {
                        eprintln!("Error saving tasks: {}", err);
                        process::exit(1);
                    });
                    println!("Marked task {} as complete.", index);
                } else {
                    eprintln!("Task index out of range.");
                }
            } else {
                eprintln!("Invalid command. Use 'add', 'list', or 'complete'.");
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Too many arguments.");
            process::exit(1);
        }
    }
}