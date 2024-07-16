use actix_web::web;

use super::handlers::{self, get_tasks};



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tasks").route(web::get().to(get_tasks)),
    );
}