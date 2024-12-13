use crate::{ApiResult, Pool};
use actix_web::{post, web, HttpResponse};
use ausgarde::parser::id::ObjectId;
use ausgarde_actix::extractor::jwt::AccessToken;
use ausgarde_codegen::protected;
use serde_json::json;
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct CreateDomainRequest {
    pub name: String,
    pub redirect_uri: String,
}

#[post("/create")]
#[protected(permissions = "domain.create")]
pub async fn create_domain(
    token: AccessToken,
    pool: Pool,
    domain: web::Json<CreateDomainRequest>,
) -> ApiResult<HttpResponse> {
    let domain = domain.into_inner();
    let con = pool.get().await?;

    let domain_id = ObjectId::new();

    _ = con
        .query_one(
            r"
			INSERT INTO
				public.domain
			(id, name, redirect_uri, owner_id)
			VALUES
			($1, $2, $3, $4)
			RETURNING
				id
		",
            &[
                &domain_id,
                &domain.name,
                &domain.redirect_uri,
                &ObjectId::from_str(&token.0.sub.unwrap()).unwrap(),
            ],
        )
        .await?;

    Ok(HttpResponse::Created().json(json!({
        "id": domain_id,
        "name": domain.name,
    })))
}
