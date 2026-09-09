use crate::task::ModelTask;
use rusqlite::{params, Connection, Result};

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

pub fn list_task(conn: &Connection) ->  Result<Vec<ModelTask>>{

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
    Ok(result )
}