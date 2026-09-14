use rusqlite::{Connection, Result};
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
