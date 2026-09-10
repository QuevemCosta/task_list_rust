use clap::Parser;
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
    Delete,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let conn = Connection::open("data.db")?;
    database::start_db(&conn)?;
    //adicionar validacao de banco

    match args.command {
        Command::Add => database::create_task(&conn, args.parms.expect("Empty task description"))?,
        Command::List => {
            let tasks = database::list_task(&conn).expect("Erro ao recuperar task");

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
        Command::Check => database::check_task(&conn, args.parms.expect("ID not found"))?,
        Command::Delete => database::delet_task(&conn, args.parms.expect("ID not found"))?,
    }
    Ok(())
}
