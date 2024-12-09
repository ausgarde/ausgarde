use actix_web::{get, HttpResponse};
use serde_json::json;

pub mod v1;

#[get("/health-check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
