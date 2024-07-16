use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};

use crate::database::database::Database;

pub async fn get_tasks(db: web::Data<Mutex<Database>>) -> impl Responder {
    match db.lock().unwrap().get_tasks() {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprintln!("Error fetching tasks: {}", e);
            HttpResponse::InternalServerError().body("Error fetching tasks")
        }
    }
}
