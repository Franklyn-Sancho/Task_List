use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::{database::operations::Database, interfaces::cli::task_list::{self, Task}};

#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub task: String,
}

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

pub async fn create_tasks(
    db: web::Data<Arc<Mutex<Database>>>,
    task: web::Json<Task>,
) -> impl Responder {
    let db = db.lock().unwrap().clone();
    match db.insert_task(&task).await {
        Ok(_) => HttpResponse::Ok().body("Task created"),
        Err(e) => {
            eprintln!("Error creating task from database: {}", e);
            HttpResponse::InternalServerError().body(format!("Error creating task: {}", e))
        }
    }
}

pub async fn get_task_by_name(
    db: web::Data<Arc<Mutex<Database>>>,
    json: web::Json<TaskQuery>,
) -> impl Responder {
    let db = db.lock().unwrap().clone();
    let task_name = json.task.clone();

    match db.get_task_by_name(&task_name).await {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}


