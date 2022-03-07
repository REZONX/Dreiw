use actix_web::{web,HttpServer,App};
use chrono::NaiveDateTime;
use routers::general_routes;
use state::AppState;
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;

mod routers;

mod handlers;

mod state;

mod models;

mod db_access;

mod errors;
//配置 route
// pub fn general_routes(cfg:&mut web::ServiceConfig) {
//     cfg.route("/health", web::get().to(health_check_handler));
// }

// //配置handler
// pub async fn health_check_handler() -> impl Responder {
//     HttpResponse::Ok().json("Actix Web Service is running!")
// }
use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub struct Course {
    pub id:i32,
    pub teacher_id:i32,
    pub name:String,
    pub time:Option<NaiveDateTime>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Nothing");
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();


    let course_rows = sqlx::query!(r#"
        select * from course where teacher_id = $1
    "#,1)
    .fetch_all(&db_pool)
    .await
    .unwrap();

    print!("{:?}",course_rows);
    let mut courses_list = vec![];
    for row in course_rows {
        courses_list.push(Course {
            id:row.id,
            teacher_id:row.teacher_id,
            name:row.name,
            time:Some(chrono::NaiveDateTime::from(row.time)),
        })
    }
    //NaiveDateTime equals with timestamp without date
    let share_data = web::Data::new(
        AppState {
            health_check_response:"I'm Ok.".to_string(),
            visit_count:Mutex::new(0),
            db:db_pool,
        }
    );
    //构建app，配置 route
    // let app = move || App::new().app_data(share_data.clone()).configure(general_routes);

    //运行 HTTP server
    // HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
    Ok(())
}

