use crate::database;
use crate::models::task::ModelTask;
use crate::repos::task_repo;
use rusqlite::{Connection, Result};
use std::io;

pub fn new_task(parms: String) -> Result<()> {
    let task: ModelTask = ModelTask {
        id: None,
        descript: parms,
        completed: false,
    };

    let conn = Connection::open("data.db")?;
    database::start_db(&conn)?;

    Ok(task_repo::create_task(&conn, &task)?)
}

pub fn list_task() -> Result<Vec<ModelTask>> {
    let conn = Connection::open("data.db")?;
    database::start_db(&conn)?;

    Ok(task_repo::list_task(&conn)?)
}
pub fn check_task(id: String) -> Result<()> {
    let conn = Connection::open("data.db")?;
    database::start_db(&conn)?;

    Ok(task_repo::check_task(&conn, id)?)
}

pub fn delet_task(id: String) -> Result<()> {
    println!("Do you really want to permanently delete the task? y/n");

     let conn = Connection::open("data.db")?;
    database::start_db(&conn)?;

    let mut confirm = String::new();
    io::stdin()
        .read_line(&mut confirm)
        .expect("Erro ao ler entrada");
    
    if confirm.to_string() == "y\n".to_string() {
        task_repo::delet_task(&conn, id)?;
    }
    else if confirm.to_string() == "n\n".to_string(){
         println!("Operation cancelled");
    }
    else {
        println!("Invalid option");
    }

    Ok(())
}
