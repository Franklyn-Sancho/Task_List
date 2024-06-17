use chrono::{NaiveDate, NaiveTime};
use rusqlite::{
    ffi::Error,
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Connection, ToSql,
};

use crate::{task_list::Task, todo_list::Priority};

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
            id TEXT PRIMARY KEY, name TEXT NOT NULL, date TEXT NOT NULL, time TEXT NOT NULL, priority TEXT NOT NULL 
        )",[], ).unwrap();
    }

    pub fn insert_task(db: &Database, task: &Task) -> Result<(), rusqlite::Error> {
        // Use parameterized query with model fields
        let sql = "INSERT INTO tasks (id, name, date, time, priority) VALUES (?1, ?2, ?3, ?4, ?5)";
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();
        let params = params![task.id, task.task, date_str, time_str, task.priority];
        db.conn.execute(sql, params)?;
        Ok(())
    }

    pub fn get_tasks(db: &Database) -> Result<Vec<(String, String, String, Priority)>, rusqlite::Error> {
        let mut stmt = db.conn.prepare("SELECT name, date, time, priority FROM tasks")?;
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
    
}
