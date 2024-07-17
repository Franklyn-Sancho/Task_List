use std::{env, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};
use crate::database::database::Database;

use super::router::init_routes;


#[actix_web::main]
pub async fn run_web() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database = Arc::new(Mutex::new(Database::new().expect("Falha ao criar pool de banco de dados")));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
