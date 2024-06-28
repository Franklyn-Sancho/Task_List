use chrono::{NaiveDate, NaiveTime};
use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Connection, ToSql,
};

use crate::task_list::{Priority, Task};

pub struct Database {
    pub conn: Connection,
}

impl ToSql for Priority {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        match self {
            Priority::Low => Ok(ToSqlOutput::from("Low")),
            Priority::Medium => Ok(ToSqlOutput::from("Medium")),
            Priority::High => Ok(ToSqlOutput::from("High")),
        }
    }
}

impl FromSql for Priority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok("Low") => Ok(Priority::Low),
            Ok("Medium") => Ok(Priority::Medium),
            Ok("High") => Ok(Priority::High),
            Ok(_) => Err(FromSqlError::InvalidType),
            Err(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl Database {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).unwrap();
        Self { conn }
    }

    pub fn create_tables(&self) {
        self.conn.execute("CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY, task TEXT NOT NULL, date TEXT NOT NULL, time TEXT NOT NULL, priority TEXT NOT NULL 
        )",[], ).unwrap();
    }

    pub fn insert_task(db: &Database, task: &Task) -> Result<(), rusqlite::Error> {
        // Use parameterized query with model fields
        let sql = "INSERT INTO tasks (id, task, date, time, priority) VALUES (?1, ?2, ?3, ?4, ?5)";
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();
        let params = params![task.id, task.task, date_str, time_str, task.priority];
        db.conn.execute(sql, params)?;
        Ok(())
    }

    pub fn task_exists(db: &Database, task_name: &str) -> Result<bool, rusqlite::Error> {
        let sql = "SELECT COUNT(*) FROM tasks WHERE task = ?1";
        let mut stmt = db.conn.prepare(sql)?;
        let count: i32 = stmt.query_row(params![task_name], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn get_tasks(
        db: &Database,
    ) -> Result<Vec<(String, String, String, Priority)>, rusqlite::Error> {
        let mut stmt = db
            .conn
            .prepare("SELECT task, date, time, priority FROM tasks")?;
        let tasks_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Priority>(3)?,
            ))
        })?;

        let mut tasks = Vec::new();
        for task in tasks_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn update_task_database(
        db: &Database,
        task_name: &str,
        task: &Task,
    ) -> Result<(), rusqlite::Error> {
        let sql = "UPDATE tasks SET task = ?1, date = ?2, time = ?3, priority = ?4 WHERE task = ?5";
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();
        let params = rusqlite::params![task.task, date_str, time_str, task.priority, task_name];
        db.conn.execute(sql, params)?;
        Ok(())
    }

    pub fn remove_task(db: &Database, name: &str) {
        match db
            .conn
            .execute("DELETE FROM tasks WHERE task = ?1", params![name])
        {
            Ok(_) => println!("The task was deleted successfully"),
            Err(e) => println!("Erro ao deletar a task {}", e),
        }
    }
}
