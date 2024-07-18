use std::{env, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};

use super::{database_web::DatabaseWeb, router::init_routes};

#[actix_web::main]
pub async fn run_web() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    /* let database = Arc::new(Mutex::new(DatabaseWeb::new().expect("Falha ao criar pool de banco de dados"))); */

    HttpServer::new(move || {
        App::new()
            /* .app_data(web::Data::new(database.clone())) */
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
