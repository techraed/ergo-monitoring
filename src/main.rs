use std::process::exit;

use anyhow::Error;

mod config;
mod service;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        exit(1);
    }
}

async fn run() -> Result<(), Error> {
    let args = args::parse()?;
    let config = config::parse(&args.config_file)?;
    service::run(config).await
}

mod args {
    use anyhow::{anyhow, Error};

    const DEFAULT_CONFIG: &str = "example.yml";

    pub(super) struct Args {
        pub(super) config_file: String,
    }

    pub(super) fn parse() -> Result<Args, Error> {
        let config_file = std::env::args().nth(1).unwrap_or_else(|| {
            format!(
                "{}/{}",
                std::option_env!("CARGO_MANIFEST_DIR")
                    .expect("internal error: no cargo manifest dir env var"),
                DEFAULT_CONFIG
            )
        });
        Ok(Args { config_file })
    }
}
