use deadpool_postgres::{Config, CreatePoolError, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub database: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn get_pool(&self) -> Result<Pool, CreatePoolError> {
        let mut cfg = Config::new();

        cfg.dbname = Some(self.database.clone());
        cfg.user = Some(self.username.clone());
        cfg.password = Some(self.password.clone());
        cfg.port = Some(self.port);
        cfg.host = Some(self.host.clone());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        cfg.create_pool(Some(Runtime::Tokio1), NoTls)
    }
}
