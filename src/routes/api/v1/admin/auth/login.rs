use crate::{error::ApiError, ApiResult, Pool};
use actix_web::{post, web::Json, HttpResponse};
use ausgarde::{
    parser::{email::Email, id::ObjectId, password::Password},
    token::{jwt::JwtBuilder, DateTimeUtc, TokenGenerator},
};
use nanoid::nanoid;
use serde_json::json;
use std::net::IpAddr;

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: Email,
    pub password: Password,
}

#[post("/login")]
pub async fn login(data: Json<LoginRequest>, pool: Pool) -> ApiResult<HttpResponse> {
    let data = data.into_inner();
    let con = pool.get().await?;

    let row = con
        .query_opt(
            r"
				SELECT
					id,
					password,
					email_verified
				FROM
					ausgarde.users
				WHERE
					email = $1
				AND
					(SELECT 1 FROM
						ausgarde.email_requests
					WHERE user_id = ausgarde.users.id
					AND type = 'email_verification'::public.request_type)
					IS NULL
					",
            &[&data.email.0],
        )
        .await?
        .ok_or(ApiError::NotFoundError("user not found".to_string()))?;

    let id: ObjectId = row.get("id");
    let password: String = row.get("password");
    let email_verified: bool = row.get("email_verified");

    if !data.password.verify_password(&password).unwrap() {
        return Err(ApiError::NotFoundError("user not found".to_string()));
    }

    if !email_verified {
        // This token is used to request a new email verification token
        // if the user has not verified their email.
        // This is a measure to combat spam attacks on the user's email.
        // Since the attacker would need to have access to the user's credentials
        let token = JwtBuilder::new()
            .sub(id.to_string())
            .iat(DateTimeUtc::now())
            .exp(DateTimeUtc::now_add_1hour())
            .aud(&["ausgarde:email-verification"])
            .encode();

        return Ok(HttpResponse::BadRequest().json(json!({
            "type": "error",
            "message": "email not verified",
            "token": token,
        })));
    }

    let session_id = nanoid!(128);

    if con
        .execute(
            r"INSERT INTO
				ausgarde.sessions (id, user_id, ip_addr, country, user_agent)
			VALUES ($1, $2, $3, $4, $5)",
            &[
                &session_id,
                &id,
                // TODO: Get the real IP address, country and user agent
                &IpAddr::from([0, 0, 0, 0]),
                &"XX",
                &"Unknown",
            ],
        )
        .await?
        == 0
    {
        return Err(ApiError::ConflictError(
            "session already exists".to_string(),
        ));
    }

    let token = JwtBuilder::new()
        .sub(id.to_string())
        .iat(DateTimeUtc::now())
        .exp(DateTimeUtc::now_add_1hour())
        .aud(&["ausgarde:session"])
        .iss(&["ausgar.de"])
        .add_custom(("sid", session_id))
        .encode();

    Ok(HttpResponse::Ok().json(json!({
        "type": "success",
        "token": token,
        "expires": DateTimeUtc::now_add_1hour().0.timestamp(),
    })))
}
