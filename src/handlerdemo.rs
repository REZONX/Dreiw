use super::state::AppState;
use crate::models::*;
use actix_web::{web,HttpResponse, http::header::HttpDate};
use crate::db_access::*;
use std::convert::TryFrom;

pub async fn health_check_handler(
    app_state:web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = 
        format!("{} {} times",health_check_response,visit_count);
        *visit_count += 1;
        HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course:web::Json<Course>,
    app_state:web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}


pub async fn get_courses_for_teacher(
    app_state:web::Data<AppState>,
    params:web::Path<(usize,)>, //代表元组里只有一个元素
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let courses = get_courses_for_teacher_db(&app_state.db,teacher_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_detail(
    app_state:web::Data<AppState>,
    params:web::Path<(usize,usize)>,
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    HttpResponse::Ok().json("Success")
}
