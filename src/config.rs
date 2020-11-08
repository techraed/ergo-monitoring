use config::{Config, File};
use thiserror::Error;

pub trait MonitoringConfig {
    // Config file keys
    const SOURCES: &'static str = "sources";

    fn get_sources(&self) -> Result<Vec<String>, ConfigError>;
}

#[derive(Debug, Clone)]
pub struct MonitoringYmlConfig {
    config: Config,
}

impl MonitoringYmlConfig {
    pub fn try_new(path_str: &str) -> Result<Self, ConfigError> {
        if !path_str.ends_with("yml") {
            return Err(ConfigError::InvalidConfigFileFormat);
        }
        let mut config = Config::new();
        config
            .merge(File::with_name(path_str))
            .expect("internal error: config was changed during processing");

        Ok(MonitoringYmlConfig { config })
    }
}

impl MonitoringConfig for MonitoringYmlConfig {
    fn get_sources(&self) -> Result<Vec<String>, ConfigError> {
        let format_source = |source| format!("http://{}/info", source);
        let sources = self
            .config
            .get_array(Self::SOURCES)
            .map_err(|_| ConfigError::NoConfigEntry)?;
        let sources: Vec<_> = sources
            .iter()
            .map(format_source)
            .collect();
        Ok(sources)
    }
}

#[derive(Error, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConfigError {
    #[error("Invalid config format")]
    InvalidConfigFileFormat,

    #[error("Config key not found")]
    NoConfigEntry,
}
