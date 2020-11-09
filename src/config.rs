//! Config file parsing logic
use std::path::Path;
use std::ffi::OsStr;

use reqwest::Url;
use thiserror::Error;

pub(super) trait IntoMonitoringConfig {
    // Config file keys
    const SOURCES: &'static str = "sources";

    fn into_monitoring_config(self) -> Result<MonitoringConfig, ConfigError>;
}

#[derive(Error, Clone, PartialEq, Eq, Debug)]
pub(super) enum ConfigError {
    #[error("Config key not found")]
    NoConfigEntry,
    #[error("Source string {0} can't be converted to url")]
    BadSourceString(String)
}

#[derive(Debug, Clone)]
pub(super) struct MonitoringConfig {
    pub(super) sources: Vec<Url>
}

pub(super) fn parse(path_str: &str) -> Result<impl IntoMonitoringConfig, ConfigError> {
    let format = get_file_format(path_str).expect("internal error: no data to extract file format");
    match format {
        "yml" => yml::parse(path_str),
        _ => unreachable!()
    }
}

fn get_file_format(path_str: &str) -> Option<&str> {
    Path::new(path_str)
        .extension()
        .and_then(OsStr::to_str)
}

mod yml {
    use reqwest::Url;
    use config::{Config, File};

    use super::{ConfigError, MonitoringConfig, IntoMonitoringConfig};

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
                .expect("internal error: config was changed during processing");

            Ok(YmlConfig(config))
        }
    }

    impl IntoMonitoringConfig for YmlConfig {
        fn into_monitoring_config(self) -> Result<MonitoringConfig, ConfigError> {
            let YmlConfig(config) = self;
            let format_source = |source| format!("http://{}/info", source);
            let source_to_url = |source: String| Url::parse(source.as_str()).map_err(|_| ConfigError::BadSourceString(source));

            let sources = config
                .get_array(Self::SOURCES)
                .map_err(|_| ConfigError::NoConfigEntry)?
                .iter()
                .map(format_source)
                .map(source_to_url)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(MonitoringConfig { sources })
        }
    }
}