use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer};
use crate::database::{config::Config, operations::Database};

use super::router::init_routes;


#[actix_web::main]
pub async fn run_web() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = Config::init_pool();
    let db = Arc::new(Mutex::new(Database { pool: pool.clone() }));

    // Chamar a função create_tables antes de iniciar o servidor
    {
        let db = db.clone();
        // Execute a criação das tabelas de forma assíncrona
        let create_table_future = async {
            let db = db.lock().unwrap();
            if let Err(e) = db.create_tables().await {
                eprintln!("Failed to create tables: {}", e);
            }
        };

        create_table_future.await;
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



