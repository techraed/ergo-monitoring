use std::collections::HashMap;

use reqwest;

use crate::config_core::{MonitoringConfig};

#[derive(Debug)]
pub struct MonitoringService<C: MonitoringConfig> {
    config: C
}

impl<C: MonitoringConfig> MonitoringService<C> {
    pub fn new(config: C) -> Self {
        MonitoringService{ config }
    }

    pub fn run(&self) -> Result<Vec<Option<u64>>, ()> {
        let mut res = vec![];
        let sources = self.config.get_sources().or(Err(()))?;
        for source in sources {
            let resp = reqwest::blocking::get(source).or(Err(()))?;
            println!("{:?}", resp);
            let json_resp = resp.json::<HashMap<String, serde_json::Value>>().or(Err(()))?;
            let b = json_resp.get("peersCount").ok_or(())?;
            res.push(b.as_u64());
        }
        Ok(res)
    }
}