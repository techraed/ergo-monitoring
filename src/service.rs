use anyhow::Error;

use crate::config::MonitoringConfig;
use crate::service::req::{InfoResp, create_client};

mod peers_number;
mod req;

pub async fn run<C: MonitoringConfig + Clone>(config: C) -> Result<(), Error> {
    peers_number::monitor(config).await?;
    Ok(())
}
