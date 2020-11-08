use std::time::Duration;


use futures::future::join_all;
use tokio::{task, time::timeout};
use anyhow::Error;
use reqwest::{get, Response, Url, ClientBuilder, Client};
use serde::Deserialize;

use crate::config_core::MonitoringConfig;
use futures::FutureExt;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct InfoResp {
    #[serde(rename(deserialize = "peersCount"))]
    peers_number: u64
}

pub async fn run<C: MonitoringConfig>(config: C) -> Result<(), Error> {
    let url_sources = {
        let sources = config
            .get_sources()
            .map_err(Error::from)?;
        to_url(sources)?
    };
    dump_peers_num(url_sources).await;
    Ok(())
}

fn to_url(sources: Vec<String>) -> Result<Vec<Url>, Error> {
    sources
        .iter()
        .map(|url| Url::parse(url).map_err(Error::from))
        .collect()
}

async fn dump_peers_num(sources: Vec<Url>) {
    let client = ClientBuilder::new().timeout(Duration::from_secs(3)).build().expect("internal error: client build failed");
    let client = Arc::new(client);
    let mut works = vec![];
    for source in sources {
        let w = task::spawn(peer_num_work(client.clone(), source));
        works.push(w);
    }
    join_all(works).await;
}

async fn peer_num_work(client: Arc<Client>, source: Url) {
    match client.get(source.as_str()).send().await {
        Err(e) => println!("An error occurred trying to reach {}", e.to_string()),
        Ok(resp) => {
            match resp.json::<InfoResp>().await {
                Err(e) => println!("An error occurred trying to parse response body from {} to JSON: {}", source, e.to_string()),
                Ok(info_resp) => println!("Source {} has peers count {}", source, info_resp.peers_number)
            }
        }
    };
}
