use crate::database;
use crate::errors::task_errors::TaskError;
use crate::models::task::Task;
use crate::repos::task_repo;
use rusqlite::{Connection, Result};
use std::io;



pub fn create_task(conn: &Connection, task_title: String) -> Result<(), TaskError> {
    let trimmed = task_title.trim();

    if trimmed.is_empty() {
        return Err(TaskError::Emptydescriptionion);
    }
    let task: Task = Task {
        id: None,
        description: trimmed.to_string(),
        completed: false,
    };

    task_repo::create_task(&conn, &task).map_err(|_| TaskError::DatabaseError)
}

pub fn list_tasks() -> Result<Vec<Task>> {
    let conn = Connection::open("data.db")?;
    database::init_database(&conn)?;

    Ok(task_repo::list_tasks(&conn)?)
}
pub fn toggle_task(id: String) -> Result<()> {
    let conn = Connection::open("data.db")?;
    database::init_database(&conn)?;

    Ok(task_repo::toggle_task(&conn, id)?)
}

pub fn delete_task(id: String) -> Result<()> {
    println!("Do you really want to permanently delete the task? y/n");

    let conn = Connection::open("data.db")?;
    database::init_database(&conn)?;

    let mut confirm = String::new();
    io::stdin()
        .read_line(&mut confirm)
        .expect("Erro ao ler entrada");

    if confirm.to_string() == "y\n".to_string() {
        task_repo::delete_task(&conn, id)?;
    } else if confirm.to_string() == "n\n".to_string() {
        println!("Operation cancelled");
    } else {
        println!("Invalid option");
    }

    Ok(())
}


//===================TESTES create_task() ==================
#[test]
fn deve_retornar_erro_quando_titulo_vazio() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    // Act
    let result = create_task(&conn, "".to_string());

    // Asser
    
    assert!(matches!(result, Err(TaskError::Emptydescriptionion)));
}

#[test]
fn deve_criar_tarefa_quando_titulo_for_valido() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    // Act
    let result = create_task(&conn, "Teste Valido".to_string());

    // Asser
    
    assert!(result.is_ok());
}

#[test]
fn deve_retornar_erro_quando_titulo_conter_apenas_espacos(){
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    // Act
    let result = create_task(&conn, "  ".to_string());

    // Asser
    
    assert!(matches!(result,
        Err(TaskError::Emptydescriptionion)
    ));
}


#[test]
fn deve_salvar_tarefa_no_banco(){
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    // Act
    let result = create_task(&conn, "Estudar Rust".to_string());

    // Asser
    
    assert!(result.is_ok());

    let tasks = task_repo::list_tasks(&conn).unwrap();
    assert_eq!(tasks.len(),1);
    assert_eq!(tasks[0].description, "Estudar Rust");
}
#[test]
fn deve_salvar_tarefa_removendo_espacos_das_extremidades() {
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    let result = create_task(&conn, "  Estudar Rust  ".to_string());
    assert!(result.is_ok());

    let tasks = task_repo::list_tasks(&conn).unwrap();
    assert_eq!(tasks[0].description, "Estudar Rust");
}