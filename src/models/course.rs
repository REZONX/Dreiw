use std::convert::TryFrom;

use chrono::NaiveDateTime;
use actix_web::{web};
use serde::Serialize;
use sqlx::FromRow;

use crate::errors::WierdError;
#[derive(Debug,Serialize,Clone,sqlx::FromRow)]
pub struct Course {
    pub id:Option<i32>,
    pub teacher_id:i32,
    pub name:String,
    pub time:Option<NaiveDateTime>,
}

pub struct CreateCourse {
    pub teacher_id:i32,
    pub name:String,
}

// impl From<web::Json<CreateCourse>> for CreateCourse {
//     fn from(course:web::Json<CreateCourse>) -> Self {
//         CreateCourse { 
//             teacher_id: course.teacher_id,
//             name: course.name
//         }
//     }
// }

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = WierdError;
    fn try_from(
        course:web::Json<CreateCourse>
    ) -> Result<Self,Self::Error>{
        Ok(
            CreateCourse {
                teacher_id:course.teacher_id,
                name:course.name.clone(),
            }
        )
    }
}