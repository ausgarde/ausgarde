use actix_web::{web::Data, HttpServer};
use ausgarde::common::config;
use std::io;

pub mod routes;

pub type Pool = Data<deadpool_postgres::Pool>;

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

    HttpServer::new(move || {
        //
        actix_web::App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::init)
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    Ok(())
}
