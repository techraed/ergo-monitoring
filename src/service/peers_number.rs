//! Getting peer numbers from provided sources

use std::sync::Arc;

use anyhow::Error;
use futures::future::join_all;
use reqwest::{Client, Url};

use crate::config::MonitoringConfig;
use crate::service::{create_client, InfoResp};

/// Monitors number of peers dumping results to std out
pub(super) async fn monitor(config: MonitoringConfig) -> Result<(), Error> {
    let peers_num = get_peers_number(config.sources).await;
    dump_peers_num(peers_num);

    Ok(())
}

async fn get_peers_number(sources: Vec<Url>) -> Vec<(u64, Url)> {
    let client = create_client(2);
    let mut tasks = Vec::with_capacity(sources.len());
    for source in sources {
        tasks.push(get_peers_from_source(client.clone(), source))
    }
    join_all(tasks)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect()
}

async fn get_peers_from_source(client: Arc<Client>, source: Url) -> Result<(u64, Url), Error> {
    let resp = client.get(source.as_str()).send().await?;
    let info_resp = resp.json::<InfoResp>().await?;
    Ok((info_resp.peers_number, source))
}

// TODO any better?
fn dump_peers_num(peers_num: Vec<(u64, Url)>) {
    println!("Peers number monitoring");
    println!("---------------------------------------------------");
    println!("|{:^36}|{:^5}|", "Peer", "Peers number");
    println!("---------------------------------------------------");
    peers_num
        .iter()
        .for_each(|(num, url)| println!("|{:^36}|{:^12}|", url, num));
    println!("---------------------------------------------------");
}
