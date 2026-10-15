//use crate::errors::task_errors::TaskError;
use crate::models::task::Task;
use rusqlite::{Connection, Result, params};

pub fn create_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, ?2)",
        params![task.description, task.completed],
    )?;

    Ok(())
}

pub fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let stmt = conn.prepare("SELECT * FROM tasks");
    let mut binding = stmt?;
    let tasks = binding.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut result: Vec<Task> = Vec::new();
    for task in tasks {
        result.push(task?);
    }
    Ok(result)
}

pub fn toggle_task(conn: &Connection, id: String) -> Result<()> {
    let search_id: i64 = id.parse().unwrap();

    let mut _task = conn.query_row(
        "SELECT id, description, completed FROM  tasks WHERE id = ?1",
        params![search_id],
        |row| {
            Ok(Task {
                id: row.get(0).expect("Id não existe"),
                description: row.get(1).expect("Dado não encontrado"),
                completed: row.get(2).expect("Dado não encontrado"),
            })
        },
    )?;
    if _task.completed == false {
        _task.completed = true;
    } else {
        _task.completed = false;
    }

    conn.execute(
        "UPDATE tasks set completed =?1 WHERE id = ?2",
        params![_task.completed, search_id],
    )?;
    println!(
        "Update successfully completed: {:?} => is {:?}",
        &_task.description, &_task.completed
    );

    Ok(())
}

pub fn delete_task(conn: &Connection, id: i64) -> Result<usize> {
    

    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])

}
