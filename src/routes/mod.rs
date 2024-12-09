use actix_web::web;
use api::v1::admin;

pub mod api;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/admin")
                    .service(admin::login::login)
                    .service(admin::signup::signup)
                    .service(admin::verify::verify),
            )
            .service(api::health_check),
    );
}
