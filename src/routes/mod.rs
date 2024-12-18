use actix_web::web;
use api::v1::{admin, domain};

pub mod api;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(web::scope("/domain").service(domain::create::create_domain))
            .service(
                web::scope("/admin").service(
                    web::scope("/auth")
                        .service(admin::auth::login::login)
                        .service(admin::auth::signup::signup)
                        .service(admin::auth::verify::verify),
                ),
            )
            .service(api::health_check),
    );
}
