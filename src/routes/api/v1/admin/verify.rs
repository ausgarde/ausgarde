use crate::Pool;
use actix_web::{post, web::Path, HttpResponse};
use ausgarde::token::{
    jwt::{jsonwebtoken::Validation, JwtBuilder},
    TokenGenerator,
};
use std::str::FromStr;
use uuid::Uuid;

#[post("/verify/{token}")]
pub async fn verify(path: Path<String>, pool: Pool) -> HttpResponse {
    let token = path.into_inner();

    let mut validation = Validation::default();
    validation.set_audience(&["ausgarde:email-verification"]);

    let token = JwtBuilder::decode(token, validation).unwrap();

    let con = pool.get().await.unwrap();

    let row = con
        .execute(
            r"
			UPDATE ausgarde.domain_manager
			SET email_verified = true
			WHERE id = $1 AND email_verification_code = $2
			",
            &[
                // Cursed code, don't do this in production
                &Uuid::from_str(&token.sub.unwrap()).unwrap(),
                &token.custom["evt"].to_string(),
            ],
        )
        .await
        .unwrap();

    if row == 0 {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}
