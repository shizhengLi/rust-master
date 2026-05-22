use std::env;
use std::fs;
use std::io;
use std::path::Path;

use rusty_tasks::{TaskFilter, TaskStore};

const STORAGE_FILE: &str = ".rusty-tasks.txt";

fn main() {
    if let Err(err) = run(env::args().skip(1), Path::new(STORAGE_FILE)) {
        eprintln!("error: {err}");
        eprintln!();
        print_help();
        std::process::exit(1);
    }
}

fn run(args: impl IntoIterator<Item = String>, storage_path: &Path) -> Result<(), String> {
    let args = args.into_iter().collect::<Vec<_>>();

    match args.first().map(String::as_str) {
        Some("add") => {
            let title = args[1..].join(" ");
            if title.trim().is_empty() {
                return Err("task title cannot be empty".to_string());
            }

            let mut store = load_store(storage_path)?;
            let task = store.add(&title);
            save_store(storage_path, &store)?;
            println!("added #{} {}", task.id, task.title);
        }
        Some("list") => {
            let filter = match args.get(1).map(String::as_str) {
                None | Some("all") => TaskFilter::All,
                Some("open") => TaskFilter::Open,
                Some("done") => TaskFilter::Done,
                Some(other) => return Err(format!("unknown list filter '{other}'")),
            };

            let store = load_store(storage_path)?;
            print_tasks(&store, filter);
        }
        Some("done") => {
            let id = args
                .get(1)
                .ok_or_else(|| "missing task id".to_string())?
                .parse::<u64>()
                .map_err(|_| "task id must be a positive number".to_string())?;

            let mut store = load_store(storage_path)?;
            let task = store
                .complete(id)
                .ok_or_else(|| format!("task #{id} does not exist"))?
                .clone();
            save_store(storage_path, &store)?;
            println!("completed #{} {}", task.id, task.title);
        }
        Some("clear") => {
            save_store(storage_path, &TaskStore::new())?;
            println!("cleared all tasks");
        }
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
        }
        Some(other) => {
            return Err(format!("unknown command '{other}'"));
        }
    }

    Ok(())
}

fn load_store(path: &Path) -> Result<TaskStore, String> {
    match fs::read_to_string(path) {
        Ok(text) => TaskStore::from_storage_text(&text),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(TaskStore::new()),
        Err(err) => Err(format!("failed to read {}: {err}", path.display())),
    }
}

fn save_store(path: &Path, store: &TaskStore) -> Result<(), String> {
    fs::write(path, store.to_storage_text())
        .map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn print_tasks(store: &TaskStore, filter: TaskFilter) {
    let tasks = store.filtered(filter);

    if tasks.is_empty() {
        println!("no tasks");
        return;
    }

    for task in tasks {
        let status = if task.done { "x" } else { " " };
        println!("[{status}] #{} {}", task.id, task.title);
    }
}

fn print_help() {
    println!("rusty-tasks - a small Rust task tracker");
    println!();
    println!("Usage:");
    println!("  cargo run -- add <title>       Add a task");
    println!("  cargo run -- list [all|open|done]");
    println!("  cargo run -- done <id>         Mark a task as done");
    println!("  cargo run -- clear             Remove all tasks");
    println!("  cargo run -- help              Show this help");
}
