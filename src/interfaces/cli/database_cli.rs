use std::error::Error;

use crate::{
    database::operations::Database,
    interfaces::cli::task_list::{Priority, Status, Task},
};

pub struct DatabaseCli {
    common: Database,
}

impl DatabaseCli {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let common = Database::new(&database_url)?;
        Ok(Self { common })
    }

    pub fn create_tables(&self) -> Result<(), Box<dyn Error>> {
        self.common.create_tables()
    }


    pub fn insert_task(&self, task: &Task) -> Result<(), Box<dyn Error>> {
        self.common.insert_task(task)
    }

    pub fn task_exists(&self, task_name: &str) -> Result<bool, Box<dyn Error>> {
        self.common.task_exists(task_name)
    }

    pub fn get_tasks(
        &self,
    ) -> Result<Vec<(String, String, String, Priority, Status)>, Box<dyn Error>> {
        self.common.get_tasks()
    }

    pub fn update_task_database(&self, task_name: &str, task: &Task) -> Result<(), Box<dyn Error>> {
        self.common.update_task_database(task_name, task)
    }

    pub fn update_task_status(&self, task: &str) -> Result<(), Box<dyn Error>> {
        self.common.update_task_status(task)
    }

    pub fn remove_task(&self, name: &str) -> Result<(), Box<dyn Error>> {
        self.common.remove_task(name)
    }
}
