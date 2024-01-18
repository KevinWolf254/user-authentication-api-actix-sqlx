use std::{ fmt::Display, fmt::Formatter, fmt::Result };

use serde::{Serialize, Deserialize};
use actix_web::{ error::ResponseError, http::StatusCode, HttpResponse};

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn new(message: Option<String>, cause: Option<String>, error_type: AppErrorType) -> Self {
        AppError { message, cause, error_type }
    }

    fn message(&self) -> String {
        match &*self {
            AppError { message: Some(message), cause: _, error_type: _ } => message.clone(),
            _ => String::from("An unexpected error occurred!"),
            
        }
    }
}

#[derive(Debug)]
pub enum AppErrorType {
    DBError,
    NotFoundError,
    BadRequestError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppResponseError {
    pub error: String,
}

impl AppResponseError {
    pub fn new(message: String) -> Self {
        AppResponseError { error: message }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.error_type {
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::BadRequestError => StatusCode::BAD_REQUEST,
            AppErrorType::DBError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppResponseError::new(self.message()))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}