use config::{Config, File};
use reqwest::Url;

pub trait MonitoringConfig {
    fn get_sources(&self) -> Result<Vec<Url>, ()>;
}

#[derive(Debug, Clone)]
pub struct MonitoringYmlConfig {
    config: Config,
}

impl MonitoringYmlConfig {
    pub fn new(path_str: &str) -> Result<Self, ()> {
        if !path_str.ends_with("yml") {
            return Err(());
        }
        let mut config = Config::new();
        config.merge(File::with_name(path_str)).or(Err(()))?;

        Ok(MonitoringYmlConfig { config })
    }

}

impl MonitoringConfig for MonitoringYmlConfig {
    fn get_sources(&self) -> Result<Vec<Url>, ()> {
        let sources = self.config.get_array("sources").or(Err(()))?;
        let format_source = |source| { format!("http://{}/info", source) };
        sources
            .iter()
            .map(|source_value| format_source(source_value))
            .map(|source_str| Url::parse(source_str.as_str()).or(Err(())))
            .collect()
    }

}