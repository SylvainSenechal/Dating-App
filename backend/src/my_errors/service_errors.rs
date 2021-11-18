use std::fmt;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

use crate::my_errors::sqlite_errors::SqliteError;

#[derive(Debug)]

pub enum ServiceError {
    ServiceError(String),
    SqliteError(SqliteError),
    ArgonError,
    JwtError,
    UnknownServiceError
}
// todo : use static str for error messayes ?
impl ServiceError {
    pub fn name(&self) -> String {
        match self {
            Self::ServiceError(_) => "Service layer error".to_string(),
            Self::SqliteError(_) => "Sqlite internal error".to_string(),
            Self::ArgonError => "Argon internal error".to_string(),
            Self::JwtError => "Jwt internal error".to_string(),
            Self::UnknownServiceError => "Unknown service layer error".to_string(),
        }
    }
}
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]

struct ErrorResponse { // todo : this is declared twice in service and sqlite errors
    code: u16,
    error_type: String,
    detailed_error: String,
}

impl actix_web::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SqliteError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ArgonError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::JwtError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnknownServiceError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            error_type: self.name(),
            detailed_error: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}