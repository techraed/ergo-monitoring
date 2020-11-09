use anyhow::Error;

use crate::config::IntoMonitoringConfig;
use crate::service::req::{InfoResp, create_client};

mod peers_number;
mod req;

pub(super) async fn run<C: IntoMonitoringConfig>(config: C) -> Result<(), Error> {
    let monitoring_config = config.into_monitoring_config()?;
    peers_number::monitor(monitoring_config).await?;
    Ok(())
}
