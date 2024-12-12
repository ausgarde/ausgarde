use crate::{error::ApiError, ApiResult, Pool};
use actix_web::{post, web::Json, HttpResponse};
use ausgarde::{
    parser::{email::Email, id::ObjectId, password::Password},
    token::{jwt::JwtBuilder, DateTimeUtc, TokenGenerator},
};
use nanoid::nanoid;

#[derive(serde::Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: Email,
    pub password: Password,
}

#[post("/signup")]
pub async fn signup(data: Json<SignupRequest>, pool: Pool) -> ApiResult<HttpResponse> {
    let data = data.into_inner();
    let con = pool.get().await?;

    let user_id = ObjectId::new();
    let email_verification_token = nanoid!(128);

    _ = con
        .query_opt(
            r"
			WITH inserted_user AS (
			INSERT INTO ausgarde.users (
					id,
					name,
					email,
					password
				)
				VALUES ($1, $2, $3, $4)
				ON CONFLICT (email) DO NOTHING
				RETURNING id
			)
			INSERT INTO ausgarde.email_requests (user_id, type, code)
			SELECT id, 'email_verification'::public.request_type, $5
			FROM inserted_user RETURNING user_id;
		",
            &[
                &user_id,
                &data.name,
                &data.email.0,
                &data.password.to_argon2_hash().unwrap(),
                &email_verification_token,
            ],
        )
        .await?
        .ok_or(ApiError::ConflictError("email already exists".to_string()))?;

    let token = JwtBuilder::new()
        .sub(user_id.to_string())
        .iat(DateTimeUtc::now())
        .exp(DateTimeUtc::now_add_1hour())
        .aud(&["ausgarde:email-verification"])
        .add_custom(("evt", email_verification_token))
        .encode();

    #[cfg(debug_assertions)]
    tracing::info!(?token, "email verification");

    // TODO: send email to the user with the token

    Ok(HttpResponse::Ok().finish())
}
