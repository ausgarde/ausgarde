use crate::{ApiResult, Pool};
use actix_web::{post, web::Path, HttpResponse};
use ausgarde::{
    parser::id::ObjectId,
    token::{
        jwt::{jsonwebtoken::Validation, JwtBuilder},
        TokenGenerator,
    },
};
use std::str::FromStr;

#[post("/verify/{token}")]
pub async fn verify(path: Path<String>, pool: Pool) -> ApiResult<HttpResponse> {
    let token = path.into_inner();

    let mut validation = Validation::default();
    validation.set_audience(&["ausgarde:email-verification"]);

    let token = JwtBuilder::decode(token, validation).unwrap();

    let con = pool.get().await?;

    let row = con
        .execute(
            r"
		WITH updated_user AS (
			UPDATE ausgarde.users
			SET email_verified = TRUE
			WHERE id = $1
			RETURNING id
		)
		DELETE FROM ausgarde.email_requests
		WHERE
			user_id = $1 AND type = 'email_verification' AND code = $2;	
			",
            &[
                // Cursed code, don't do this in production
                &ObjectId::from_str(&token.sub.unwrap()).unwrap(),
                &token.custom["evt"].as_str().unwrap(),
            ],
        )
        .await?;

    if row == 0 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
