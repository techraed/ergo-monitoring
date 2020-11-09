//! Config file parsing logic

use std::convert::TryInto;
use std::ffi::OsStr;
use std::path::Path;

use reqwest::Url;
use thiserror::Error;

// All possible errors during config file deserialization to `MonitoringConfig`
#[derive(Error, Debug)]
pub(super) enum ConfigError {
    #[error("Config file parsing failed: {0}")]
    ConfigParsingError(String),
    #[error("Config key not found")]
    NoConfigEntry,
    #[error("Source string {0} can't be converted to url")]
    BadSourceString(String),
}

#[derive(Debug, Clone)]
pub(super) struct MonitoringConfig {
    pub(super) sources: Vec<Url>,
}

pub(super) fn parse(path_str: &str) -> Result<impl TryInto<MonitoringConfig, Error = ConfigError>, ConfigError> {
    let format = get_file_format(path_str).expect("internal error: no data to extract file format");
    match format {
        "yml" => yml::parse(path_str),
        _ => unreachable!(),
    }
}

fn get_file_format(path_str: &str) -> Option<&str> {
    Path::new(path_str).extension().and_then(OsStr::to_str)
}

mod yml {
    use std::convert::TryInto;

    use config::{Config, File};
    use reqwest::Url;

    use super::{keys, ConfigError, MonitoringConfig};

    #[derive(Debug, Clone)]
    pub(super) struct YmlConfig(Config);

    pub(super) fn parse(path_str: &str) -> Result<YmlConfig, ConfigError> {
        YmlConfig::try_new(path_str)
    }

    impl YmlConfig {
        fn try_new(path_str: &str) -> Result<Self, ConfigError> {
            let mut config = Config::new();
            config
                .merge(File::with_name(path_str))
                .map_err(|e| ConfigError::ConfigParsingError(e.to_string()))?;

            Ok(YmlConfig(config))
        }
    }

    impl TryInto<MonitoringConfig> for YmlConfig {
        type Error = ConfigError;

        fn try_into(self) -> Result<MonitoringConfig, Self::Error> {
            let YmlConfig(config) = self;
            let format_source = |source| format!("http://{}/info", source);
            let source_to_url = |source: String| {
                Url::parse(source.as_str()).map_err(|_| ConfigError::BadSourceString(source))
            };

            let sources = config
                .get_array(keys::SOURCES)
                .map_err(|_| ConfigError::NoConfigEntry)?
                .iter()
                .map(format_source)
                .map(source_to_url)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(MonitoringConfig { sources })
        }
    }
}

mod keys {
    //! Config file keys

    pub(super) const SOURCES: &'static str = "sources";
}
