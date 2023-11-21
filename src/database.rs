use sea_orm::ConnectOptions;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
}

impl DatabaseSettings {
    pub fn get_connection_options(uri: &str) -> ConnectOptions {
        let mut opt = ConnectOptions::new(uri.to_owned());
        opt.max_connections(100).sqlx_logging(true);

        opt
    }
}
