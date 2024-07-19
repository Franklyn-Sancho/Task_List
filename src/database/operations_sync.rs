use postgres::NoTls;
// src/db_operations_sync.rs
use r2d2_postgres::r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use std::error::Error;

use super::operations::{create_tables, get_tasks, insert_task, remove_task, task_exists, update_task_database, update_task_status};

pub fn create_tables_sync(client: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<(), Box<dyn Error>> {
    tokio::runtime::Runtime::new()?.block_on(create_tables(client))
}

pub fn insert_task_sync(client: &mut PooledConnection<PostgresConnectionManager<NoTls>>, task: &Task) -> Result<(), Box<dyn Error>> {
    tokio::runtime::Runtime::new()?.block_on(insert_task(client, task))
}

pub fn task_exists_sync(client: &mut PooledConnection<PostgresConnectionManager<NoTls>>, task_name: &str) -> Result<bool, Box<dyn Error>> {
    tokio::runtime::Runtime::new()?.block_on(task_exists(client, task_name))
}

pub fn get_tasks_sync(client: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<(String, String, String, Priority, Status)>, Box<dyn Error>> {
    tokio::runtime::Runtime::new()?.block_on(get_tasks(client))
}

pub fn update_task_database_sync(client: &mut PooledConnection<PostgresConnectionManager<NoTls>>, task_name: &str, task: &Task) -> Result<(), Box<dyn Error>> {
    tokio::runtime::Runtime::new()?.block_on(update_task_database(client, task_name, task))
}

pub fn update_task_status_sync(client: &mut PooledConnection<PostgresConnectionManager<NoTls>>, task: &str) -> Result<(), Box<dyn Error>> {
    tokio::runtime::Runtime::new()?.block_on(update_task_status(client, task))
}

pub fn remove_task_sync(client: &mut PooledConnection<PostgresConnectionManager<NoTls>>, name: &str) -> Result<(), Box<dyn Error>> {
    tokio::runtime::Runtime::new()?.block_on(remove_task(client, name))
}
