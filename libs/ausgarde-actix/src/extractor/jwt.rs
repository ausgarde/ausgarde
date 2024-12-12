use actix_web::{error, web::Data, FromRequest, HttpRequest};
use ausgarde::token::{
    jwt::{jsonwebtoken::Validation, JwtBuilder},
    TokenGenerator,
};
use std::future::{ready, Ready};

#[derive(Clone)]
pub struct JwtValidator(pub Validation);

/// Extracts the Access token from the Authorization header, and validates it.
///
/// It will return an error to the client, if the token is not present or if it is invalid.
///
/// Don't forget to add `JwtValidator` to your app_data, in order to use this extractor.
/// If you don't add `JwtValidator` to your app_data, it will use the default validation in debug mode, and panic in release mode.
///
/// # Example
/// ```no_run
/// use actix_web::{web, HttpResponse, get};
///
/// #[get("/")]
/// async fn index(token: AccessToken) -> HttpResponse {
///    HttpResponse::Ok().finish()
/// }
/// ```
pub struct AccessToken(pub JwtBuilder);

impl FromRequest for AccessToken {
    type Error = actix_web::error::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let validation = match req.app_data::<Data<JwtValidator>>() {
            Some(v) => v.0.clone(),
            None => {
                #[cfg(debug_assertions)]
                {
                    tracing::error!("No JwtValidator found in app_data. Using default validation.");
                    tracing::warn!("Please add JwtValidator to your app_data.");
                }

                #[cfg(not(debug_assertions))]
                {
                    panic!("No JwtValidator found in app_data");
                }

                Validation::default()
            }
        };

        // Extracts the token from the Authorization header
        // And transforms the `HeaderValue` into a `&str` and then splits it by whitespace
        // And then takes the second element of the split
        let token = match req
            .headers()
            .get("authorization")
            .and_then(|x| x.to_str().unwrap().split_whitespace().nth(1))
            .ok_or(error::ErrorUnauthorized("No token"))
        {
            Ok(v) => v,
            Err(e) => {
                return ready(Err(e));
            }
        };

        tracing::info!(?token);
        tracing::info!(?validation);

        match JwtBuilder::decode(token, validation) {
            Some(claims) => ready(Ok(AccessToken(claims))),
            None => ready(Err(error::ErrorUnauthorized("Invalid token"))),
        }
    }
}
