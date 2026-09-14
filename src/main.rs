use clap::Parser;

mod infra;
mod models;
mod repos;
mod services;

use crate::infra::database;
use crate::services::task_services;
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
    database::start_db(&conn)?;
    //adicionar validacao de banco

    match args.command {
        Command::Add => task_services::new_task(args.parms.expect("Empty task description"))?,
        Command::List => {
            let tasks = task_services::list_task().expect("Erro ao recuperar task");

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
        Command::Check => task_services::check_task(args.parms.expect("ID not found"))?,
        Command::Delete => task_services::delet_task(args.parms.expect("ID not found"))?,
    }
    Ok(())
}
