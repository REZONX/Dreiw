use actix_web::{error,http::StatusCode,HttpRequest,Result, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug,Serialize)]
pub enum WierdError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug,Serialize)]
pub struct WierdErrorResponse {
    error_message:String,
}

impl WierdError {
    fn error_response(&self) -> String {
        match self {
            WierdError::DBError(msg) => {
                println!("Database error occurred:{:?}",msg);
                "Database error".into()
            }
            WierdError::ActixError(msg) => {
                println!("Actix error occurred:{:?}",msg);
                "Internal server error".into()
            }
            WierdError::NotFound(msg) => {
                println!("not found error occurred:{:?}",msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for WierdError {
    fn status_code(&self) -> StatusCode {
        match self {
            WierdError::DBError(msg) | WierdError::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            WierdError::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(WierdErrorResponse {
            error_message:self.error_response(),
        })
    }
}

impl fmt::Display for WierdError {
    fn fmt(&self,f:&mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(f,"{}",self)
    }
}

impl From<SQLxError> for WierdError {
    fn from(err:SQLxError) -> Self {
        WierdError::DBError(err.to_string())
    }
}

impl From<actix_web::error::Error> for WierdError {
    fn from(err:actix_web::error::Error) -> Self {
        WierdError::ActixError(err.to_string())
    }
}
