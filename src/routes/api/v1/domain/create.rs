use std::str::FromStr;

use crate::{ApiResult, Pool};
use actix_web::{post, web, HttpResponse};
use ausgarde::parser::id::ObjectId;
use ausgarde_actix::extractor::jwt::AccessToken;

#[derive(serde::Deserialize)]
pub struct CreateDomainRequest {
    pub name: String,
    pub redirect_uri: String,
}

#[post("/create")]
pub async fn create_domain(
    at: AccessToken,
    pool: Pool,
    domain: web::Json<CreateDomainRequest>,
) -> ApiResult<HttpResponse> {
    let domain = domain.into_inner();
    let con = pool.get().await?;

    let row = con
        .query_one(
            r"
			INSERT INTO
				ausgarde.domains
			(id, name, redirect_uri, owner_id)
			VALUES
			($1, $2, $3, $4)
			RETURNING
				id
		",
            &[
                &ObjectId::new(),
                &domain.name,
                &domain.redirect_uri,
                &ObjectId::from_str(&at.0.sub.unwrap()).unwrap(),
            ],
        )
        .await?;

    Ok(HttpResponse::NotImplemented().finish())
}
