use std::env;

use bytes::BytesMut;
use dotenv::dotenv;
use postgres::{types::{to_sql_checked, FromSql, IsNull, ToSql, Type}, Client, Error, NoTls};

use crate::interfaces::cli::task_list::{Priority, Status, Task};

pub struct Database {
    client: Client
}

impl ToSql for Priority {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let value = match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };
        out.extend_from_slice(value.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for Priority {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn  std::error::Error + Sync + Send>> {
        let value = std::str::from_utf8(raw)?;
        
        match value {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            _ => Err("Invalid priority value".into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}


impl<'a> FromSql<'a> for Status {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let value = std::str::from_utf8(raw)?;
        
        match value {
            "Pending" => Ok(Status::Pendent),
            "Completed" => Ok(Status::Completed),
            _ => Err("Invalid status value".into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}

impl ToSql for Status {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let value = match self {
            Status::Pendent => "Pending",
            Status::Completed => "Completed",
        };
        out.extend_from_slice(value.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}


impl Database {
    pub fn new() -> Result<Self, Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        let client = Client::connect(&database_url, NoTls)?;
        Ok(Self { client })

    }

    pub fn create_tables(&mut self) -> Result<(), Error> {
        self.client.execute(
            "
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                task TEXT NOT NULL,
                date TEXT NOT NULL,
                time TEXT NOT NULL,
                priority TEXT NOT NULL,
                status TEXT NOT NULL
            )
            ", &[])?;

            Ok(())
    }

    pub fn insert_task(&mut self, task: &Task) -> Result<(), Error> {
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();

        self.client.execute(
            "INSERT INTO tasks (id, task, date, time, priority, status) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&task.id, &task.task, &date_str, &time_str, &task.priority, &task.status],
        )?;
        Ok(())
    }

    pub fn task_exists(&mut self, task_name: &str) -> Result<bool, Error> {
        let row = self.client.query_one(
            "SELECT COUNT(*) FROM tasks WHERE task = $1", 
            &[&task_name])?;
            let count: i64 = row.get(0);
            Ok(count > 0)
    }
    
    pub fn get_tasks(&mut self) -> Result<Vec<(String, String, String, Priority, Status)>, Error> {

        let rows = self.client.query("SELECT task, date, time, priority, status FROM tasks", &[])?;
        let mut tasks = Vec::new();

        for row in rows {
            let task = row.get::<_, String>(0);
            let date = row.get::<_, String>(1);
            let time = row.get::<_, String>(2);
            let priority = row.get::<_, Priority>(3);
            let status = row.get::<_, Status>(4);
            tasks.push((task, date, time, priority, status));
        }

        Ok(tasks)
    }

    pub fn update_task_database(&mut self, task_name: &str, task: &Task) -> Result<(), Error> {
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();

        let _ = self.client.execute(
            "UPDATE tasks SET task = $1, date = $2, time = $3, priority = $4, status = $5 WHERE task = $6", 
            &[&task.task, &date_str, &time_str, &task.priority, &task.status, &task_name]);
        
        Ok(())
    }


    pub fn update_task_status(&mut self, task: &str) -> Result<(), Error> {
        let status_completed = Status::Completed;

        let _ = self.client.execute(
            "UPDATE tasks SET status = $1 WHERE task = $2", 
            &[&status_completed, &task]);

        Ok(())
    }


    pub fn remove_task(&mut self, name: &str) {
        match self
            .client
            .execute("DELETE FROM tasks WHERE task = $1", &[&name])
        {
            Ok(_) => println!("The task was deleted successfully"),
            Err(e) => println!("delete task error: {}", e),
        }
    } 
        

}

   

