use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;
use std::{fmt, ops::Add};

#[derive(Debug)]
pub enum SqliteError {
    NotFound,
    UnknownSqliteError,
    SqliteFailure(libsqlite3_sys::Error),
    SqliteFailureExplained(libsqlite3_sys::Error, String),
    SqliteFailureNoText,
}

impl SqliteError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => "Ressource not found".to_string(),
            Self::SqliteFailureExplained(sqlite_failure_detail, explaination) => {
                sqlite_failure_detail.to_string().add(" : ").add(explaination)
            }
            Self::SqliteFailure(sqlite_failure_detail) => sqlite_failure_detail.to_string(),
            Self::SqliteFailureNoText => {
                "Something fucked up in the database, it's not your fault dud".to_string()
            }
            Self::UnknownSqliteError => "Unknown sqlite error".to_string(),
        }
    }
}
impl fmt::Display for SqliteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Serialize)]

struct ErrorResponse { // todo : this is declared twice in service and sqlite errors
    code: u16,
    error: String,
    message: String,
}
impl actix_web::ResponseError for SqliteError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::SqliteFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SqliteFailureExplained(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SqliteFailureNoText => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnknownSqliteError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn map_sqlite_error(e: rusqlite::Error) -> SqliteError {
    // Todo : Add this to the logger
    println!("map sqlite error found : {:?}", e);

    match e {
        rusqlite::Error::QueryReturnedNoRows => SqliteError::NotFound,
        rusqlite::Error::SqliteFailure(sqlite_failure_detail, Some(explaination)) => {
            SqliteError::SqliteFailureExplained(sqlite_failure_detail, explaination)
        }
        rusqlite::Error::SqliteFailure(sqlite_failure_detail, None) => {
            SqliteError::SqliteFailure(sqlite_failure_detail)
        }
        rusqlite::Error::InvalidColumnIndex(_) => SqliteError::SqliteFailureNoText,
        rusqlite::Error::InvalidColumnType(_, _, _) => SqliteError::SqliteFailureNoText,
        rusqlite::Error::InvalidColumnName(_) => SqliteError::SqliteFailureNoText,

        _ => SqliteError::UnknownSqliteError,
    }
}
