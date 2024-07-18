use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};

use super::database_web::DatabaseWeb;


pub async fn get_tasks_json(db: web::Data<Arc<Mutex<DatabaseWeb>>>) -> impl Responder {
    let db = db.lock().unwrap().clone();
    match db.get_tasks().await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            println!("Error fetching tasks from database: {}", e);
            HttpResponse::InternalServerError().body(format!("Error fetching tasks: {}", e))
        }
    }
}



