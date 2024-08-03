use actix_web::web;

use super::handlers::{self, create_tasks, get_task_by_name, get_tasks_json};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tasks").route(web::get().to(get_tasks_json)));
    cfg.service(web::resource("/create").route(web::post().to(create_tasks)));
    cfg.service(web::resource("/get_by_name").route(web::post().to(get_task_by_name)));
}
