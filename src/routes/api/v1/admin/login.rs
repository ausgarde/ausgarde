use actix_web::{post, web::Json, HttpResponse};
use ausgarde::parser::{email::Email, password::Password};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: Email,
    pub password: Password,
}

#[post("/login")]
pub async fn login(data: Json<LoginRequest>) -> HttpResponse {
    _ = data;

    todo!()
}
