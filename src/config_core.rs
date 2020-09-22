use config::{Config, File};
use thiserror::Error;

type Result<T> = std::result::Result<T, ConfigError>;

pub trait MonitoringConfig {
    // Config file keys
    const SOURCES: &'static str = "sources";

    fn get_sources(&self) -> Result<Vec<String>>;
}

#[derive(Debug, Clone)]
pub struct MonitoringYmlConfig {
    config: Config,
}

impl MonitoringYmlConfig {
    pub fn new(path_str: &str) -> Result<Self> {
        if !path_str.ends_with("yml") {
            return Err(ConfigError::InvalidFormat);
        }
        let mut config = Config::new();
        config
            .merge(File::with_name(path_str))
            .expect("config can't be changed");

        Ok(MonitoringYmlConfig { config })
    }
}

impl MonitoringConfig for MonitoringYmlConfig {
    fn get_sources(&self) -> Result<Vec<String>> {
        let sources = self
            .config
            .get_array(Self::SOURCES)
            .or(Err(ConfigError::KeyNotFound))?;
        let format_source = |source| format!("http://{}/info", source);
        sources
            .iter()
            .map(|source_value| Ok(format_source(source_value)))
            .collect()
    }
}

#[derive(Error, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConfigError {
    #[error("Invalid config format")]
    InvalidFormat,

    #[error("Config key not found")]
    KeyNotFound,
}
