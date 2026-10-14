use crate::errors::task_errors::TaskError;
use crate::infra::database;
use crate::models::task::Task;
use crate::repos::task_repo;
use rusqlite::{Connection, Result};

//=========================CREATE_TASK()===============================================
pub fn create_task(conn: &Connection, task_title: String) -> Result<(), TaskError> {
    let trimmed = task_title.trim();

    if trimmed.is_empty() {
        return Err(TaskError::EmptyDescription);
    }
    if trimmed.chars().count() >= 150 {
        return Err(TaskError::MaxLenChars);
    }
    let task: Task = Task {
        id: None,
        description: trimmed.to_string(),
        completed: false,
    };

    task_repo::create_task(&conn, &task).map_err(|_| TaskError::DatabaseError)
}
//=========================FIM CREATE_TASK()=================================

//=========================TASK_LIST()=================================
pub fn list_tasks(conn: &Connection) -> Result<Vec<Task>, TaskError> {
    database::init_database(conn).map_err(|_| TaskError::DatabaseError)?;

    let tasks = task_repo::list_tasks(conn).map_err(|_| TaskError::ListTasksFailed)?;

    Ok(tasks)
}
//=========================FIM TASK_LIST()=================================

//=========================TAGGLE_LIST()=================================
pub fn toggle_task(conn: &Connection, id: String) -> Result<(), TaskError> {
    task_repo::toggle_task(conn, id).map_err(|e_| TaskError::TaskNotFound)
}
//=========================FIM TAGGLE_LIST()=================================

//=========================DELET_LIST()=====================================
pub fn delete_task(conn: &Connection, id: String) -> Result<(), TaskError> {
    task_repo::delete_task(conn, id).map_err(|_e| TaskError::DeleteError);
    Ok(())
}
//=========================FIM DELET_LIST()=====================================

//===================INICIO TESTES create_task() ==============================
#[test]
fn deve_retornar_erro_quando_titulo_vazio() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    // Act
    let result = create_task(&conn, "".to_string());

    // Asser

    assert!(matches!(result, Err(TaskError::EmptyDescription)));
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
fn deve_retornar_erro_quando_titulo_conter_apenas_espacos() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    // Act
    let result = create_task(&conn, "  ".to_string());

    // Asser

    assert!(matches!(result, Err(TaskError::EmptyDescription)));
}

#[test]
fn deve_salvar_tarefa_no_banco() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    // Act
    let result = create_task(&conn, "Estudar Rust".to_string());

    // Asser

    assert!(result.is_ok());

    let tasks = task_repo::list_tasks(&conn).unwrap();
    assert_eq!(tasks.len(), 1);
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
#[test]
fn test_create_task_with_maximum_length_title() {
    let conn = Connection::open_in_memory().unwrap();
    database::init_database(&conn);

    let result = create_task(&conn, "A".repeat(150));
    assert!(matches!(result, Err(TaskError::MaxLenChars)));
}
#[test]
fn test_create_task_with_just_under_maximum_length_title() {
    let conn = Connection::open_in_memory().unwrap();
    database::init_database(&conn);

    let result = create_task(&conn, "A".repeat(149));
    assert!(result.is_ok());
}
//===================FIM TESTES create_task() ================================

//======================INICIO TESTES LIST_TASK()=========================
#[test]
fn testa_conexao_positiva_com_banco() {
    // Arrange
    let conn = Connection::open_in_memory();
    assert!(conn.is_ok());
}

#[test]
fn testa_erro_na_conexao_com_banco() {
    let conn = Connection::open("/task_list");

    assert!(conn.is_err());
}

#[test]
fn testa_listagem_com_dois_elementos() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    create_task(&conn, "Teste 01".to_string());
    create_task(&conn, "Teste 02".to_string());

    let result = list_tasks(&conn);
    assert!(result.is_ok());

    let tasks = result.unwrap();
    assert_eq!(tasks.len(), 2);
}

#[test]
fn para_quando_retorna_uma_lista_de_tasks_vazia() {
    // Arrange
    let conn = Connection::open_in_memory().unwrap();
    let _ = database::init_database(&conn);

    let result = list_tasks(&conn);
    assert!(result.is_ok());
    let tasks = result.unwrap();
    assert_eq!(tasks.len(), 0);
}
#[test]
fn deve_retornar_erro_quando_consulta_das_tarefas_falhar() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE VIEW tasks AS SELECT 1;", ()).unwrap();
    let result = list_tasks(&conn);
    dbg!(&result);
    assert!(result.is_err());
}
