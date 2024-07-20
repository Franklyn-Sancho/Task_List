use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

use crate::database::operations::Database;

pub async fn get_tasks_json(db: web::Data<Arc<Mutex<Database>>>) -> impl Responder {
    let db = db.lock().unwrap().clone();

    match db.get_tasks().await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprintln!("Error fetching tasks from database: {}", e);
            HttpResponse::InternalServerError().body(format!("Error fetching tasks: {}", e))
        }
    }
}




