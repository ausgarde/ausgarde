use ausgarde::token::jwt::jsonwebtoken::Validation;
use ausgarde::token::jwt::JwtBuilder;
use ausgarde::token::{DateTimeUtc, TokenGenerator};

fn main() {
    std::env::set_var("AUSGARDE_SIGNING_TOKEN", "secret");

    let token = JwtBuilder::new()
        .sub("some-id")
        .exp(DateTimeUtc::now_add_15min())
        .add_custom(("key", "value"))
        .add_custom(("int", 123))
        .add_custom(("boolean", true))
        .add_custom(("array", vec!["hello", "world"]))
        .jti("jwt-id")
        .iss(&["ausgarde"])
        .aud(&["audience"])
        .iat(DateTimeUtc::now())
        .encode();

    println!("Encoded: {token}");

    let mut val = Validation::default();
    val.set_issuer(&["ausgarde", "secret"]);
    val.set_audience(&["audience"]);

    println!("verified: {}", JwtBuilder::decode(token, val).is_some());
}
