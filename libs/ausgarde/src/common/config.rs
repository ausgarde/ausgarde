use cconfig::Environment;
use config as cconfig;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[cfg(feature = "database")]
    pub pg: deadpool_postgres::Config,

    pub server: Server,
}

#[derive(Deserialize)]
pub struct Server {
    pub port: u16,
    pub bind: String,
}

impl Config {
    pub fn from_env() -> Self {
        cconfig::Config::builder()
            .add_source(Environment::default().separator("__"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
