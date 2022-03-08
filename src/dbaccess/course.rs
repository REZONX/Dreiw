use actix_web::{web,HttpResponse};
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;
use crate::state::AppState;
use crate::errors::WierdError;
use crate::models::{course::Course};

pub async fn get_courses_for_teacher_db(
    pool:&PgPool,
    teacher_id:i32
) ->Result<Vec<Course>,WierdError>{
    let course_rows = sqlx::query!(r#"
        SELECT *
        FROM course
        WHERE teacher_id = $1
    "#,teacher_id)
    .fetch_all(pool)
    .await?;
    let mut courses:Vec<Course> = vec![];
    for row in course_rows {
        courses.push(
            Course {
                id:Some(row.id),
                teacher_id:row.teacher_id,
                name:row.name,
                time:Some(row.time),
            }
        )
    }
    Ok(courses)

}

pub async fn push_new_course_db(
    pool:&PgPool,
    new_course:Course
) -> Course {
    let row = sqlx::query!(
        r#"
            INSERT INTO course (teacher_id,name)
            VALUES ($1,$2)
            RETURNING id,teacher_id,name,time
        "#,
        new_course.teacher_id,
        new_course.name,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    Course {
        id:Some(row.id),
        teacher_id:row.teacher_id,
        name:row.name,
        time:Some(row.time),
    }
}

