//! Manage configuration of application settings with `config` crate using a type that implements
//! `serde`'s `Deserialize` trait

#[derive(serde::Deserialize)]
pub struct Settings {
    /// Database connection parameters
    pub database: DatabaseSettings,
    /// Application port `actix-web` is listening for incoming requests on
    pub application_port: u16,
}

/// Database settings
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

/// Read applicaiton settings from a configuration file named `configuration.yaml`
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "/Users/jgentile/src/zero2prod/configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    // Try to convert the configuration values it read into Settings struct
    settings.try_deserialize::<Settings>()
}
