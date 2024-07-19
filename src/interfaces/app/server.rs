use std::{env, error::Error, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};

use crate::database::connections::create_pool_web;

use super::router::init_routes;

#[actix_web::main]
pub async fn run_web() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = create_pool_web(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
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
