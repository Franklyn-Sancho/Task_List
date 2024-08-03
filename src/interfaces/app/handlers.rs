use actix_web::{web, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::{database::operations::Database, interfaces::cli::task_list::{self, Priority, Status, Task}};

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(serde::de::Error::custom("Invalid date format")),
    }
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let time_str = String::deserialize(deserializer)?;
    match NaiveTime::parse_from_str(&time_str, "%H:%M:%S") {
        Ok(time) => Ok(time),
        Err(_) => Err(serde::de::Error::custom("Invalid time format")),
    }
}

#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub task: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskWeb {
    pub task: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    #[serde(deserialize_with = "deserialize_time")]
    pub time: NaiveTime,
    pub priority: Priority,
    pub status: Status,
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
    task: web::Json<TaskWeb>,
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

pub async fn update_task(
    db: web::Data<Arc<Mutex<Database>>>,
    path: web::Path<String>,
    json: web::Json<TaskWeb>,
) -> impl Responder {
    let db = db.lock().unwrap().clone();
    let task_name = path.into_inner();
    let task = json.into_inner();

    match db.update_task_database(&task_name, &task).await {
        Ok(_) => HttpResponse::Ok().body("Task updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub async fn remove_task(
    db: web::Data<Arc<Mutex<Database>>>,
    path: web::Path<String>
) -> impl Responder {
    let db = db.lock().unwrap().clone();
    let task_name = path.into_inner();

    match db.remove_task(&task_name).await {
        Ok(_) => HttpResponse::Ok().body("Task Removed Successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}


