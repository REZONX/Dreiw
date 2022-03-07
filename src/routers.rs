use actix_web::web;
use chrono::NaiveDateTime;
use super::handlers::*;

pub fn general_routes(cfg:&mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg:&mut web::ServiceConfig){
    cfg
    .service(web::scope("/course"));
    // .route("", web::post().to(new_course));
}