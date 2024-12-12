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

    /// A custom error type that allows for a custom status code to be returned.
    /// The first string is the kind of the error, the second string is the message.
    /// The kind will get transformed to uppercase. The status code is optional, if not provided
    /// the status code will be `INTERNAL_SERVER_ERROR`.
    #[error("{0}")]
    Custom(String, String, Option<actix_web::http::StatusCode>),
}

impl ApiError {
    pub fn custom(
        kind: &str,
        message: &str,
        status_code: Option<actix_web::http::StatusCode>,
    ) -> Self {
        ApiError::Custom(kind.to_string(), message.to_string(), status_code)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::DatabasePoolError(_) | ApiError::DatabaseError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::ConflictError(_) => actix_web::http::StatusCode::CONFLICT,
            ApiError::NotFoundError(_) => actix_web::http::StatusCode::NOT_FOUND,
            ApiError::Custom(_, _, status_code) => {
                status_code.unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
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
            ApiError::Custom(kind, message, _) => {
                body["kind"] = json!(kind.to_uppercase());
                body["message"] = json!(message);
            }
        }

        actix_web::HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(body.to_string())
    }
}
