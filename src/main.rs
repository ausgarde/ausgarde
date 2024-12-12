use actix_web::{web::Data, HttpServer};
use ausgarde::{common::config, token::jwt::jsonwebtoken::Validation};
use ausgarde_actix::extractor::jwt::JwtValidator;
use std::io;

pub mod error;
pub mod routes;

pub type Pool = Data<deadpool_postgres::Pool>;
pub type ApiResult<T> = Result<T, error::ApiError>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().init();

    tracing::info!("starting up");

    let config = config::Config::from_env();

    tracing::info!("creating database pool");

    let pool = config
        .pg
        .create_pool(
            Some(deadpool_postgres::Runtime::Tokio1),
            tokio_postgres::NoTls,
        )
        .unwrap();

    tracing::info!("starting server");

    let mut validator = Validation::default();

    validator.set_audience(&["ausgarde:session"]);
    validator.set_issuer(&["ausgar.de"]);

    let validator = JwtValidator(validator);

    HttpServer::new(move || {
        //
        actix_web::App::new()
            .app_data(Data::new(validator.clone()))
            .app_data(Data::new(pool.clone()))
            .configure(routes::init)
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    Ok(())
}
