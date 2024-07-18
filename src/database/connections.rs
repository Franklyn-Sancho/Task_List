use std::error::Error;

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use postgres::NoTls;
use r2d2_postgres::{PostgresConnectionManager};


pub async fn create_pool_web(database_url: &str) -> Result<Pool, Box<dyn Error>> {
    let mut cfg = Config::new();
    cfg.dbname = Some(database_url.to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

pub fn create_pool_cli(database_url: &str) -> Result<r2d2::Pool<PostgresConnectionManager<NoTls>>, Box<dyn Error>> {
    let manager = PostgresConnectionManager::new(database_url.parse()?, NoTls);
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}