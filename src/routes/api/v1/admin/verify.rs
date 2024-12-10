use crate::{ApiResult, Pool};
use actix_web::{post, web::Path, HttpResponse};
use ausgarde::token::{
    jwt::{jsonwebtoken::Validation, JwtBuilder},
    TokenGenerator,
};
use std::str::FromStr;
use uuid::Uuid;

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
			UPDATE ausgarde.domain_manager
			SET 
				email_verified = true,
				email_verified_at = now(),
				email_verification_code = null
			WHERE id = $1 AND email_verification_code = $2
			",
            &[
                // Cursed code, don't do this in production
                &Uuid::from_str(&token.sub.unwrap()).unwrap(),
                &token.custom["evt"].as_str().unwrap(),
            ],
        )
        .await?;

    if row == 0 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
