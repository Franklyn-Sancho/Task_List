use chrono::{NaiveDate, NaiveTime};
use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;

use crate::interfaces::{app::handlers::TaskWeb, cli::task_list::{Priority, Status, Task}};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_client(&self) -> Result<deadpool_postgres::Client, Box<dyn Error>> {
        let client = self.pool.get().await?;
        Ok(client)
    }

    pub async fn create_tables(&self) -> Result<(), Box<dyn Error>> {
        let client = self.get_client().await?;
        client
            .batch_execute(
                "
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                task TEXT NOT NULL,
                date TEXT NOT NULL,
                time TEXT NOT NULL,
                priority TEXT NOT NULL,
                status TEXT NOT NULL
            )
        ",
            )
            .await?;
        Ok(())
    }

    pub async fn insert_task(&self, task: &TaskWeb) -> Result<(), Box<dyn Error>> {
        let client = self.get_client().await?;
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();

        // Gerar UUID v4
        let id = Uuid::new_v4().to_string();

        client.execute(
            "INSERT INTO tasks (id, task, date, time, priority, status) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&id, &task.task, &date_str, &time_str, &task.priority, &task.status],
        ).await?;
        Ok(())
    }

    pub async fn task_exists(&self, task_name: &str) -> Result<bool, Box<dyn Error>> {
        let client = self.get_client().await?;
        let row = client
            .query_one("SELECT COUNT(*) FROM tasks WHERE task = $1", &[&task_name])
            .await?;
        let count: i64 = row.get(0);
        Ok(count > 0)
    }

    pub async fn get_tasks(
        &self,
    ) -> Result<Vec<(String, String, String, Priority, Status)>, Box<dyn Error>> {
        let client = self.get_client().await?;
        let rows = client
            .query("SELECT task, date, time, priority, status FROM tasks", &[])
            .await?;
        let mut tasks = Vec::new();

        for row in rows {
            let task_name: String = row.get(0);
            let date: String = row.get(1);
            let time: String = row.get(2);
            let priority: Priority = row.get(3);
            let status: Status = row.get(4);
            tasks.push((task_name, date, time, priority, status));
        }

        Ok(tasks)
    }

    pub async fn get_task_by_name(
        &self,
        task_name: &str,
    ) -> Result<Vec<(String, String, String, Priority, Status)>, Box<dyn Error>> {
        let client = self.get_client().await?;
        let rows = client
            .query(
                "SELECT task, date, time, priority, status FROM tasks WHERE task = $1",
                &[&task_name],
            )
            .await?;
        let mut task = Vec::new();

        for row in rows {
            let task_name: String = row.get(0);
            let date: String = row.get(1);
            let time: String = row.get(2);
            let priority: Priority = row.get(3);
            let status: Status = row.get(4);
            task.push((task_name, date, time, priority, status));
        }

        Ok(task)
    }

    pub async fn update_task_database(
        &self,
        task_name: &str,
        task: &TaskWeb,
    ) -> Result<(), Box<dyn Error>> {
        let client = self.get_client().await?;
        let date_str = task.date.format("%Y-%m-%d").to_string();
        let time_str = task.time.format("%H:%M:%S").to_string();

        client.execute(
            "UPDATE tasks SET task = $1, date = $2, time = $3, priority = $4, status = $5 WHERE task = $6", 
            &[&task.task, &date_str, &time_str, &task.priority, &task.status, &task_name],
        ).await?;

        Ok(())
    }

    pub async fn update_task_status(&self, task: &str) -> Result<(), Box<dyn Error>> {
        let client = self.get_client().await?;
        let status_completed = "Completed";

        client
            .execute(
                "UPDATE tasks SET status = $1 WHERE task = $2",
                &[&status_completed, &task],
            )
            .await?;

        Ok(())
    }

    pub async fn remove_task(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let client = self.get_client().await?;
        client
            .execute("DELETE FROM tasks WHERE task = $1", &[&name])
            .await?;
        println!("The task was deleted successfully");
        Ok(())
    }
}
