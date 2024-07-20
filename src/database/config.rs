use config::ConfigError;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
}


impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }

    pub fn print_pg_config(&self) {
        println!("PostgreSQL Config: {:?}", self.pg);
    }
}

/* pub fn create_pool(database_url: &str) -> Result<Pool, Box<dyn Error>> {
    let mut cfg = Config::new();
    cfg.dbname = Some(database_url.to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(Some(DeadpoolRuntime::Tokio1), NoTls)?;
    Ok(pool)
}

pub fn create_pool_from_env() -> Result<Pool, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    let mut cfg = Config::new();
    cfg.dbname = Some(database_url);
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
} */
