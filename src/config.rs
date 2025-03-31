use config::ConfigError;
use serde::Deserialize;

use crate::db::pg_conn::PgConn;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    // pub database_url: String,
    pub instance_domain: String,
    pub force_auth_fetch: bool,
    pub bind_address: String,
    pub contact_email: String,
    pub port: u16,
    pub outbox_pagnation_size: u64,

    pub pg_user: String,
    pub pg_password: String,
    pub pg_host: String,
    pub pg_port: u16,
    pub pg_dbname: String,
}

impl Config {
    pub fn create_conn(&self) -> PgConn {
        let db_config = deadpool_postgres::Config {
            user: Some(self.pg_user.clone()),
            password: Some(self.pg_password.clone()),
            host: Some(self.pg_host.clone()),
            dbname: Some(self.pg_dbname.clone()),

            ..Default::default()
        };

        let pool = db_config.create_pool(None, tokio_postgres::NoTls).unwrap();
        PgConn { db: pool }
    }
}

pub fn get_config() -> Result<Config, ConfigError> {
    let settings = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("ap_config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::default())
        .build();

    let settings = match settings {
        Ok(x) => x,
        Err(x) => {
            return Err(x);
        }
    };

    let config = match settings.try_deserialize::<Config>() {
        Ok(config) => config,
        Err(error) => {
            return Err(error);
        }
    };
    Ok(config)
}
