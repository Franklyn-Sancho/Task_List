/* use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Connection, ToSql,
};

use crate::task_list::{Priority, Status, Task}; */

/* pub struct Database {
    pub conn: Connection,
} */

/* impl ToSql for Priority {
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

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        match self {
            Status::Pendent => Ok(ToSqlOutput::from("Pendent")),
            Status::Completed => Ok(ToSqlOutput::from("Completed")),
        }
    }
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok("Pendent") => Ok(Status::Pendent),
            Ok("Completed") => Ok(Status::Completed),
            Ok(_) => Err(FromSqlError::InvalidType),
            Err(_) => Err(FromSqlError::InvalidType),
        }
    }
} */

/* impl Database {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).unwrap();
        Self { conn }
    }

    pub fn create_tables(&self) {
        self.conn.execute("CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY, task TEXT NOT NULL, date TEXT NOT NULL, time TEXT NOT NULL, priority TEXT NOT NULL, status TEXT NOT NULL
        )",[], ).unwrap();
    }

    pub fn insert_task(&self, task: &Task) -> Result<(), rusqlite::Error> {
        // Use parameterized query with model fields
        let sql = "INSERT INTO tasks (id, task, date, time, priority, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();
        let params = params![
            task.id,
            task.task,
            date_str,
            time_str,
            task.priority,
            task.status
        ];
        self.conn.execute(sql, params)?;
        Ok(())
    }

    pub fn task_exists(&self, task_name: &str) -> Result<bool, rusqlite::Error> {
        let sql = "SELECT COUNT(*) FROM tasks WHERE task = ?1";
        let mut stmt = self.conn.prepare(sql)?;
        let count: i32 = stmt.query_row(params![task_name], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn get_tasks(
        &self,
    ) -> Result<Vec<(String, String, String, Priority, Status)>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT task, date, time, priority, status FROM tasks")?;
        let tasks_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Priority>(3)?,
                row.get::<_, Status>(4)?,
            ))
        })?;

        let mut tasks = Vec::new();
        for task in tasks_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn update_task_database(
        &self,
        task_name: &str,
        task: &Task,
    ) -> Result<(), rusqlite::Error> {
        let sql = "UPDATE tasks SET task = ?1, date = ?2, time = ?3, priority = ?4, status = ?5 WHERE task = ?5";
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();
        let params = rusqlite::params![
            task.task,
            date_str,
            time_str,
            task.priority,
            task.status,
            task_name
        ];
        self.conn.execute(sql, params)?;
        Ok(())
    }

    pub fn update_task_status(&self, task: &str) -> Result<(), rusqlite::Error> {
        let sql = "UPDATE tasks SET status = ?1 WHERE task = ?2";
        let status_completed = Status::Completed;
        let params = rusqlite::params![status_completed, task];
        self.conn.execute(sql, params)?;
        Ok(())
    }

    pub fn remove_task(&self, name: &str) {
        match self
            .conn
            .execute("DELETE FROM tasks WHERE task = ?1", params![name])
        {
            Ok(_) => println!("The task was deleted successfully"),
            Err(e) => println!("delete task error: {}", e),
        }
    }

    
} */

/* pub fn remove_task_by_datetime(&self, date: NaiveDate, time: NaiveTime) {
        let date_str = date.format("%Y-%m-%d").to_string();
        let time_str = time.format("%H:%M:%S").to_string();

        let sql = "
        DELETE FROM tasks
        WHERE date < $1
            OR (date = $1 AND time <= $2)";
        let params = [&date_str, &time_str];

        match self.conn.execute(sql, params) {
            Ok(_) => println!("Old tasks deleted successfully"),
            Err(e) => println!("Delete task error: {}", e),
        }
    } */