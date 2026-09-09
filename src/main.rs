use clap::Parser;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use uuid::Uuid;

mod database;
mod task;


use rusqlite::{Connection, Result};

#[derive(Parser)]
struct Cli {
    command: Command,
    parms: Option<String>,
}
#[derive(clap::ValueEnum, Clone)]
enum Command {
    Add,
    List,
    Check,
    Delite,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: uuid::Uuid,
    descript: String,
    completed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    clean();

    let conn = Connection::open("data.db")?;
    database::start_db(&conn)?;
    //adicionar validacao de banco

    match args.command {
        Command::Add => database::create_task(&conn, args.parms.expect("Empty task description"))?,
        Command::List => {
            let tasks = database::list_task(&conn).expect("Erro ao recuperar task");

            clean();
            println!("+{}+", "-".repeat(45));
            println!("|{} TASK LIST {}|", " ".repeat(17), " ".repeat(17));

            for task in tasks {
                println!("+{}+", "-".repeat(45));
                println!(
                    "| ID:{:?}\n| Description: {}\n| Completed: {}",
                    task.id, task.descript, task.completed
                );
            }
            println!("+{}+", "-".repeat(45));
        }
        Command::Check => check_task(args.parms.expect("ID not found"))?,
        Command::Delite => todo!(),
    }
    Ok(())
}

fn clean() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn list_task() -> Result<Vec<Task>, Box<dyn Error>> {
    let path = File::open("dados.json")?;

    let reader = BufReader::new(path);

    let tasks: Vec<Task> = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Task>(&line).unwrap())
        .collect();
    Ok(tasks)
}

fn check_task(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let id = Uuid::parse_str(&id).expect("Invalid ID: expected a UUID");

    let mut tasks: Vec<Task> = list_task()
        .map(|list| {
            list.into_iter()
                .filter(|task| !task.descript.is_empty())
                .collect::<Vec<Task>>()
        })
        .unwrap_or_else(|_| Vec::new());

    if let Some(task) = tasks.iter_mut().find(|t| t.id.eq(&id)) {
        if !task.completed {
            task.completed = true;
            println!("Task: {} - OK", task.descript);
        } else {
            task.completed = false;
            println!("Task: {} - Not completed", task.descript)
        }

        let arquivo: Result<File, io::Error> = OpenOptions::new()
            .create(true)
            .write(true)
            .open("dados.json");

        let json_data = serde_json::to_string(&tasks).unwrap();

        let _salvo = writeln!(arquivo?, "{}", json_data);
    }

    Ok(())
}
