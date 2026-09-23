use clap::Parser;

mod errors;
mod infra;
mod models;
mod repos;
mod services;

use crate::infra::database;
use crate::services::task_service;
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
    Delete,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let conn = Connection::open("data.db")?;
    database::init_database(&conn)?;
    //adicionar validacao de banco

    match args.command {
        Command::Add => {
            task_service::create_task(&conn, args.parms.expect("Empty task descriptionion"))
                .expect("Erro")
        }
        Command::List => {
            let tasks = task_service::list_tasks(&conn).expect("Erro ao recuperar task");

            println!("+{}+", "-".repeat(45));
            println!("|{} TASK LIST {}|", " ".repeat(17), " ".repeat(17));

            for task in tasks {
                println!("+{}+", "-".repeat(45));
                println!(
                    "| ID:{:?}\n| descriptionion: {}\n| Completed: {}",
                    task.id, task.description, task.completed
                );
            }
            println!("+{}+", "-".repeat(45));
        }
        Command::Check => task_service::toggle_task(args.parms.expect("ID not found"))?,
        Command::Delete => task_service::delete_task(args.parms.expect("ID not found"))?,
    }
    Ok(())
}
