use anyhow::Error;
use ergo_monitoring::{MonitoringYmlConfig};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{}", e);
    }
}

async fn run() -> Result<(), Error> {
    let config = MonitoringYmlConfig::new("config.yml")?;
    ergo_monitoring::run(config).await
}

// TODO
// 1. вызовы бинаря из любой директории не должен фэйлить парсинг пути config.yml
