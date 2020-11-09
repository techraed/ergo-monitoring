use anyhow::Error;

use crate::config::IntoMonitoringConfig;
use crate::service::req::{create_client, InfoResp};

mod peers_number;
mod req;

pub(super) async fn run<C: IntoMonitoringConfig>(config: C) -> Result<(), Error> {
    let monitoring_config = config.into_monitoring_config()?;

    println!("STARTING MONITORING SERVICES");

    peers_number::monitor(monitoring_config).await?;
    Ok(())
}
