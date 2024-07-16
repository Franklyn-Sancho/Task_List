use std::sync::Mutex;

use actix_web::{web, App, HttpServer, Responder};

use crate::database::database::Database;

use super::router::init_routes;

// Função que retorna "Hello, World!" como resposta

#[actix_web::main]
pub async fn run_web() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .configure(init_routes) // Configura as rotas
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
