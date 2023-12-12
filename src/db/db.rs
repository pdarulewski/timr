use crate::model::task::Task;
use rusqlite::{params, Connection, Result, Statement};

pub fn connect() -> Result<Connection> {
    return Connection::open("tasks.db");
}

pub fn create_tasks_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                status TEXT NOT NULL,
                started_at TEXT,
                completed_at TEXT
            )",
        [],
    )?;

    Ok(())
}

pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (name, description, status, started_at, completed_at)
             VALUES (?, ?, ?, ?, ?)",
        params![
            task.name,
            task.description,
            task.status.to_string(),
            task.started_at.map(|dt| dt.to_string()),
            task.completed_at.map(|dt| dt.to_string())
        ],
    )?;

    Ok(())
}

pub fn delete_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?", params![task.id])?;

    Ok(())
}

pub fn start_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET status = ?, started_at = ? WHERE id = ?",
        params![
            task.status.to_string(),
            task.started_at.map(|dt| dt.to_string()),
            task.id
        ],
    )?;

    Ok(())
}

pub fn complete_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET status = ?, completed_at = ? WHERE id = ?",
        params![
            task.status.to_string(),
            task.completed_at.map(|dt| dt.to_string()),
            task.id
        ],
    )?;

    Ok(())
}

pub fn list_tasks(conn: &Connection) -> Result<Statement> {
    let stmt = conn.prepare(
        "SELECT id, name, description, status, started_at, completed_at FROM tasks ORDER BY id ASC",
    )?;

    Ok(stmt)
}
