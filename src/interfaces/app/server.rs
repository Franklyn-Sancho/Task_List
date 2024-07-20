use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use deadpool_postgres::Runtime;
use postgres::NoTls;

use crate::database::{config::Config, operations::Database};

use super::router::init_routes;
use dotenv::dotenv;

#[actix_web::main]
pub async fn run_web() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dotenv().ok();
    let cfg = Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).unwrap();
    let db = Arc::new(Mutex::new(Database { pool: pool.clone() })); // Use pool.clone() aqui

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Certifique-se de usar Data::new aqui
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/* pub async fn initialize_database() -> Result<Arc<Mutex<DatabaseWeb>>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database = Arc::new(Mutex::new(DatabaseWeb::new().await?));
    Ok(database)
}

pub async fn start_server(database: Arc<Mutex<DatabaseWeb>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 */
