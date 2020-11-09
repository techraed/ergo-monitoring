//! Monitoring service core

use std::convert::TryInto;
use std::sync::Arc;

use anyhow::Error;

use futures::future::join_all;
use reqwest::{Client, Url};

use crate::config::{MonitoringConfig, ConfigError};

use self::req::{create_client, InfoResp};

pub(super) async fn run<C: TryInto<MonitoringConfig, Error = ConfigError>>(config: C) -> Result<(), Error> {
    let config = config.try_into()?;

    println!("STARTING MONITORING SERVICE");
    let data = get_peer_data(config.sources).await;
    dump_peer_data(data);

    Ok(())
}

async fn get_peer_data(sources: Vec<Url>) -> Vec<(InfoResp, Url)> {
    let client = create_client(2);
    let mut tasks = Vec::with_capacity(sources.len());
    for source in sources {
        tasks.push(get_data_from_source(client.clone(), source))
    }
    join_all(tasks)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect()
}

async fn get_data_from_source(client: Arc<Client>, source: Url) -> Result<(InfoResp, Url), Error> {
    let resp = client.get(source.as_str()).send().await?;
    let info_resp = resp.json::<InfoResp>().await?;
    Ok((info_resp, source))
}

// TODO any better?
fn dump_peer_data(peers_num: Vec<(InfoResp, Url)>) {
    println!("Peers number monitoring");
    println!("--------------------------------------------------------------------------------------------------------");
    println!(
        "|{:^36}|{}|{}|{}|{}|",
        "Peer", " Peers number ", " Headers height ", " Full height ", " Unconfirmed count "
    );
    println!("--------------------------------------------------------------------------------------------------------");
    peers_num.iter().for_each(|(ir, url)| {
        println!(
            "|{:^36}|{:^14}|{:^16}|{:^13}|{:^19}|",
            url, ir.peers_number, ir.headers_height, ir.full_height, ir.unconfirmed_count
        )
    });
    println!("--------------------------------------------------------------------------------------------------------");
}

mod req {
    //! Request/response useful data and functions.

    use std::sync::Arc;
    use std::time::Duration;

    use reqwest::{Client, ClientBuilder};
    use serde::Deserialize;

    /// Deserialized json response from /info request to provided source
    #[derive(Debug, Deserialize)]
    pub(super) struct InfoResp {
        #[serde(rename(deserialize = "peersCount"))]
        pub(super) peers_number: u64,
        #[serde(rename(deserialize = "headersHeight"))]
        pub(super) headers_height: u64,
        #[serde(rename(deserialize = "fullHeight"))]
        pub(super) full_height: u64,
        #[serde(rename(deserialize = "unconfirmedCount"))]
        pub(super) unconfirmed_count: u64,
    }

    /// Reference counted, thread safe, cloneable client, which fails requests if timeout is met
    pub(super) fn create_client(timeout_secs: u64) -> Arc<Client> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("internal error: client build failed");
        Arc::new(client)
    }
}
