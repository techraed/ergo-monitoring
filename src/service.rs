use std::collections::HashMap;

use reqwest::{blocking::get, Url};
use thiserror::Error;

use crate::config_core::MonitoringConfig;

type Result<T> = std::result::Result<T, ServiceError>;

pub fn run<C: MonitoringConfig>(config: C) -> Result<Vec<u64>> {
    let url_sources = {
        let sources = config
            .get_sources()
            .map_err(|err| ServiceError::CannotInitializeService(err.to_string()))?;
        to_url(sources)?
    };
    Ok(get_peers_number(url_sources))
}

fn to_url(sources: Vec<String>) -> Result<Vec<Url>> {
    sources
        .iter()
        .map(|url| Url::parse(url).or(Err(ServiceError::CannotConvertData)))
        .collect()
}

fn get_peers_number(sources: Vec<Url>) -> Vec<u64> {
    let res = vec![];
    for source in sources {
        let response = get(source);
        println!("{:?}", response);
    }
    res
}

#[derive(Error, Clone, PartialEq, Eq, Debug)]
pub enum ServiceError {
    #[error("Can't initialize service {0}")]
    CannotInitializeService(String),

    #[error("Config key not found")]
    KeyNotFound,

    #[error("Config data can't be converted to proper type")]
    CannotConvertData,
}
