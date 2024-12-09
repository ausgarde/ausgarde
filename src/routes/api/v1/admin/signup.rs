use actix_web::{post, web::Json, HttpResponse};
use ausgarde::{
    parser::{email::Email, password::Password},
    token::{jwt::JwtBuilder, DateTimeUtc, TokenGenerator},
};
use nanoid::nanoid;
use uuid::Uuid;

use crate::Pool;

#[derive(serde::Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: Email,
    pub password: Password,
}

#[post("/signup")]
pub async fn signup(data: Json<SignupRequest>, pool: Pool) -> HttpResponse {
    let data = data.into_inner();
    let con = pool.get().await.unwrap();

    let row = con
        .query_opt(
            r"
		INSERT INTO ausgarde.domain_manager (id, name, email, password, email_verification_code)
		VALUES (uuid_generate_v4(), $1, $2, $3, $4) ON CONFLICT (email) DO NOTHING
		RETURNING id, email_verification_code
		",
            &[
                &data.name,
                &data.email.0,
                &data.password.to_argon2_hash().unwrap(),
                &nanoid!(128),
            ],
        )
        .await
        .unwrap()
        .unwrap(); // TODO: create a error enum, and use `ok_or` instead of `unwrap`

    let id: Uuid = row.get("id");
    let email_verification_token: String = row.get("email_verification_code");

    let token = JwtBuilder::new()
        .sub(id.to_string())
        .iat(DateTimeUtc::now())
        .exp(DateTimeUtc::now_add_1hour())
        .aud(&["ausgarde:email-verification"])
        .add_custom(("evt", email_verification_token))
        .encode();

    #[cfg(debug_assertions)]
    tracing::info!(?token, "email verification");

    // TODO: send email to the user with the token

    HttpResponse::Ok().finish()
}
