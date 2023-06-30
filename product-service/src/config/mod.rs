use serde::Deserialize;

use self::env::Environment;

mod env;

#[derive(Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Configs {
    pub app: AppConfig,
}

pub fn get_config() -> Result<Configs, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    let enviroment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let enviroment_filename = format!("{}.yaml", enviroment.as_str());

    // Init config reader
    let config = config::Config::builder()
        .add_source(config::File::from(config_dir.join("default.yaml")))
        .add_source(config::File::from(config_dir.join(enviroment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    config.try_deserialize::<Configs>()
}
