use anyhow::Error;

mod config;
mod service;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{}", e);
    }
}

async fn run() -> Result<(), Error> {
    let config = config::MonitoringYmlConfig::try_new("config.yml")?;
    service::run(config).await
}

// TODO
// 1. calls to binary from any dir must work (should not fail parsing config.yml)
// 2. pretty output
// 3. design should be ready for multiple tasks
// 4. try spawning background tasks (discuss with Kushti)
// 5. general clean-ups
// 6. better config handling: first parse with fails and then supply it to `run`

