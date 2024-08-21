pub mod task {
    use clap::Parser;
    use serde::{Deserialize, Serialize};
    use std::{
        fs::{metadata, File},
        io::{Read, Write},
        str::FromStr,
    };

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Task {
        title: String,
        description: String,
    }

    #[derive(Parser, Clone, Debug)]
    pub enum Action {
        ADD,
        DEL,
        LIST,
    }

    impl FromStr for Action {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "add" => Ok(Action::ADD),
                "del" => Ok(Action::DEL),
                "list" => Ok(Action::LIST),
                _ => Err(String::from("Invalid action")),
            }
        }
    }

    #[derive(Parser, Debug)]
    pub struct Args {
        pub action: Action,
        title: Option<String>,
        description: Option<String>,
    }

    pub fn add_task(tasks: &mut Vec<Task>, args: &Args) {
        let title = match &args.title {
            Some(t) if !t.is_empty() => t.clone(),
            _ => {
                println!("Title is required");
                return;
            }
        };
        let description = match &args.description {
            Some(d) if !d.is_empty() => d.clone(),
            _ => {
                println!("Description is required");
                return;
            }
        };

        if tasks.iter().any(|task| task.title == title) {
            println!("Task already exists!");
            return;
        }
        tasks.push(Task { title, description });
        write_file(tasks, args);
    }

    pub fn del_task(tasks: &mut Vec<Task>, args: &Args) {
        let title = match &args.title {
            Some(t) if !t.is_empty() => t.clone(),
            _ => {
                println!("Title is required");
                return;
            }
        };

        if tasks.iter().any(|task| task.title == title) {
            tasks.retain(|task| task.title != title);
            write_file(tasks, args)
        } else {
            println!("Task not found!");
            return;
        };
    }

    fn print_tasks(tasks: &mut Vec<Task>) {
        println!("{:?}", tasks);
    }

    pub fn get_tasks() -> Vec<Task> {
        let tasks: Vec<Task> = if metadata("tasks.json").is_ok() {
            let mut file = File::open("tasks.json").expect("Failed to open file");
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("Failed to read file");
            serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };
        return tasks;
    }

    pub fn perform_action(tasks: &mut Vec<Task>) {
        let args = Args::parse();
        match args.action {
            Action::ADD => add_task(tasks, &args),
            Action::DEL => del_task(tasks, &args),
            Action::LIST => print_tasks(tasks),
        };
    }

    fn write_file(tasks: &mut Vec<Task>, args: &Args) {
        let serialized = serde_json::to_string(tasks).expect("Failed to create JSON");
        let mut file = File::create("tasks.json").expect("Failed to open file");
        file.write_all(serialized.as_bytes())
            .expect("Failed to write file");
        println!("Task {:?} successful!", args.action)
    }
}
