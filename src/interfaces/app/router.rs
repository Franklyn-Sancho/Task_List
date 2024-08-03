use actix_web::web;

use super::handlers::{self, create_tasks, get_task_by_name, get_tasks_json, remove_task, update_task};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tasks").route(web::get().to(get_tasks_json)));
    cfg.service(web::resource("/create").route(web::post().to(create_tasks)));
    cfg.service(web::resource("/get_by_name").route(web::post().to(get_task_by_name)));
    cfg.service(web::resource("/update/{task_name}").route(web::put().to(update_task)));
    cfg.service(web::resource("/delete/{task_name}").route(web::delete().to(remove_task)));
}
