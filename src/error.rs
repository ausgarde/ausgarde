use actix_web::ResponseError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("a database error occurred. {0}")]
    DatabasePoolError(#[from] deadpool_postgres::PoolError),

    #[error("a database error occurred. {0}")]
    DatabaseError(#[from] tokio_postgres::Error),

    #[error("{0}")]
    ConflictError(String),

    #[error("{0}")]
    NotFoundError(String),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::DatabasePoolError(_) | ApiError::DatabaseError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::ConflictError(_) => actix_web::http::StatusCode::CONFLICT,
            ApiError::NotFoundError(_) => actix_web::http::StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let mut body = json!({
            "type": "error",
            "message": self.to_string(),
        });

        match self {
            ApiError::DatabasePoolError(_) | ApiError::DatabaseError(_) => {
                body["kind"] = json!("DATABASE_ERROR")
            }
            ApiError::ConflictError(_) => body["kind"] = json!("CONFLICT_ERROR"),
            ApiError::NotFoundError(_) => body["kind"] = json!("NOT_FOUND_ERROR"),
        }

        actix_web::HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(body.to_string())
    }
}
