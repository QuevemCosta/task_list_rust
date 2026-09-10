use crate::task::ModelTask;
use rusqlite::{Connection, Result, params};

pub fn start_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        descript TEXT NOT NULL,
        completed INTEGER
    );",
        (),
    )?;
    Ok(())
}

pub fn create_task(conn: &Connection, parms: String) -> Result<()> {
    let task: ModelTask = ModelTask {
        id: None,
        descript: parms,
        completed: false,
    };
    conn.execute(
        "INSERT INTO tasks (descript, completed) VALUES (?1, ?2)",
        params![task.descript, task.completed],
    )?;
    Ok(())
}

pub fn list_task(conn: &Connection) -> Result<Vec<ModelTask>> {
    let mut stmt = conn.prepare("SELECT * FROM tasks")?;

    let tasks = stmt.query_map([], |row| {
        Ok(ModelTask {
            id: row.get(0)?,
            descript: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut result: Vec<ModelTask> = Vec::new();
    for task in tasks {
        result.push(task?);
    }
    Ok(result)
}
pub fn check_task(conn: &Connection, id: String) -> Result<()> {
    let search_id: i64 = id.parse().unwrap();

    let mut _task = conn.query_row(
        "SELECT id, descript, completed FROM  tasks WHERE id = ?1",
        params![search_id],
        |row| {
            Ok(ModelTask {
                id: row.get(0).expect("Id não existe"),
                descript: row.get(1).expect("Dado não encontrado"),
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
        &_task.descript, &_task.completed
    );

    Ok(())
}

pub fn delet_task(conn: &Connection, id: String) -> Result<()> {
    let search_id: i64 = id.parse().unwrap();

    let lines = conn.execute("DELETE FROM tasks WHERE id = ?1", params![search_id])?;

    if lines == 0 {
        println!("To task was found – there were no changes to the task list")
    }else{
        println!("Task successfully removed.")
    }
    Ok(())
}
