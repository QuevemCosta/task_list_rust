use crate::database;
use crate::errors::task_erros::TaskError;
use crate::models::task::ModelTask;
use crate::repos::task_repo;
use rusqlite::{Connection, Result};
use std::io;



pub fn new_task(conn: &Connection, task_title: String) -> Result<(), TaskError> {
    if task_title.trim().is_empty() {
        return Err(TaskError::EmptyDescription);
    }
    let task: ModelTask = ModelTask {
        id: None,
        descript: task_title,
        completed: false,
    };

    task_repo::create_task(&conn, &task).map_err(|_| TaskError::DatabaseError)
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
    } else if confirm.to_string() == "n\n".to_string() {
        println!("Operation cancelled");
    } else {
        println!("Invalid option");
    }

    Ok(())
}

#[test]
fn deve_retornar_erro_quando_titulo_vazio() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::start_db(&conn);

    // Act
    let result = new_task(&conn, "".to_string());

    // Asser
    
    assert!(matches!(result, Err(TaskError::EmptyDescription)));
}

#[test]
fn deve_criar_tarefa_quando_titulo_for_valido() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::start_db(&conn);

    // Act
    let result = new_task(&conn, "Teste Valido".to_string());

    // Asser
    
    assert!(result.is_ok());
}

#[test]
fn deve_retornar_erro_quando_titulo_conter_apenas_espacos(){
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::start_db(&conn);

    // Act
    let result = new_task(&conn, "  ".to_string());

    // Asser
    
    assert!(matches!(result,
        Err(TaskError::EmptyDescription)
    ));
}


#[test]
fn deve_salvar_tarefa_no_banco(){
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::start_db(&conn);

    // Act
    let result = new_task(&conn, "Estudar Rust".to_string());

    // Asser
    
    assert!(result.is_ok());

    let tasks = task_repo::list_task(&conn).unwrap();
    assert_eq!(tasks.len(),1);
    assert_eq!(tasks[0].descript, "Estudar Rust");
}
