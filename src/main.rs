use std::collections::HashMap;
use ausgarde::token::bearer::BearerToken;
use ausgarde::token::TokenSigner;
use serde_json::json;

#[actix_web::main]
async fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("starting up");
}
