use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};

use crate::database::database::Database;

pub async fn get_tasks_json(db: web::Data<Arc<Mutex<Database>>>) -> impl Responder {
    let db = db.lock().unwrap().clone();
    match db.get_tasks() {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            println!("Error fetching tasks from database: {}", e);
            HttpResponse::InternalServerError().body(format!("Error fetching tasks: {}", e))
        }
    }
}

pub async fn some_handler(db: web::Data<Database>) -> impl Responder {
    // Utilize o pool de conexões dentro dos handlers
    let task_exists = db.task_exists("Some task").unwrap_or(false);

    if task_exists {
        HttpResponse::Ok().body("Task exists")
    } else {
        HttpResponse::Ok().body("Task does not exist")
    }
}
