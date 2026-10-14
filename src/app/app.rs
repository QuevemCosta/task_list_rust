use rusqlite::Connection;
use crate::Cli;
use crate::Commands;
use crate::task_service;

 use crate::models::task::Task;


pub fn app(cli: Cli, conn: &Connection) -> Result<(), Box<dyn std::error::Error>>{
    match &cli.command {
        Commands::Add { description } => {
            task_service::create_task(&conn, description.to_string());
        }
        Commands::List => {
            let tasks: Vec<Task> = task_service::list_tasks(&conn).unwrap();
            view_list(tasks);
        },
        Commands::Check { id } => {
            task_service::toggle_task(&conn, id.to_string());
        }
        Commands::Delete { id } => {
            task_service::delete_task(&conn, id.to_string());
        }
    }
    Ok(())
}

fn view_list(tasks: Vec<Task>){
    for task in tasks {
        println!("ID: {:?}", task.id);
        println!("DESCRIPTION: {}",task.description);
        println!("STATUS: {}", task.completed);
    }
}