use std::error::Error;

use deadpool_postgres::Pool;
use tokio::runtime::Runtime;

use crate::{
    database::{self, operations::Database},
    interfaces::cli::task_list::{Priority, Status, Task},
};

pub struct DatabaseCli {
    database: Database,
    runtime: Runtime,
}

impl DatabaseCli {
    pub fn new(pool: Pool) -> Result<Self, Box<dyn Error>> {
        let runtime = Runtime::new()?;
        let database = Database::new(pool);
        Ok(Self { database, runtime })
    }

    pub fn create_tables(&self) -> Result<(), Box<dyn Error>> {
        self.runtime.block_on(self.database.create_tables())?;
        Ok(())
    }

    pub fn insert_task(&self, task: &Task) -> Result<(), Box<dyn Error>> {
        self.runtime.block_on(self.database.insert_task(task))?;
        Ok(())
    }

    pub fn task_exists(&self, task_name: &str) -> Result<bool, Box<dyn Error>> {
        let exists = self.runtime.block_on(self.database.task_exists(task_name))?;
        Ok(exists)
    }
    
    pub fn get_tasks(&self) -> Result<Vec<(String, String, String, Priority, Status)>, Box<dyn Error>> {
        let tasks = self.runtime.block_on(self.database.get_tasks())?;
        Ok(tasks)
    }

    pub fn update_task_database(&self, task_name: &str, task: &Task) -> Result<(), Box<dyn Error>> {
        self.runtime.block_on(self.database.update_task_database(task_name, task))?;
        Ok(())
    }

    pub fn update_task_status(&self, task: &str) -> Result<(), Box<dyn Error>> {
        self.runtime.block_on(self.database.update_task_status(task))?;
        Ok(())
    }

    pub fn remove_task(&self, name: &str) -> Result<(), Box<dyn Error>> {
        self.runtime.block_on(self.database.remove_task(name))?;
        Ok(())
    }
}
