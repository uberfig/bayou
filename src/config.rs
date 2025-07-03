use config::ConfigError;
use serde::Deserialize;

use crate::db::pg_conn::PgConn;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// domain the instance is running on
    /// eg bayou.town
    pub instance_domain: String,
    /// ip address to bind to when running
    /// eg 127.0.0.1
    pub bind_address: String,
    /// port that this will be running on
    pub port: u16,

    /// allow users to just sign up freely.
    /// note this will not affect users using
    /// an invite to sign up
    pub open_signups: bool,
    pub allow_applications: bool,

    /// max file upload size for a standard user in mb
    pub max_standard_upload_size: usize,
    pub max_superuser_upload_size: Option<usize>,
    pub upload_memory_limit: usize,

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
        .add_source(config::File::with_name("config"))
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
