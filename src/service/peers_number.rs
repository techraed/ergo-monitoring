use std::sync::Arc;

use anyhow::Error;
use futures::future::join_all;
use reqwest::{Url, Client};

use crate::config::MonitoringConfig;
use crate::service::{InfoResp, create_client};

/// Monitors number of peers dumping results to stdout
pub(super) async fn monitor(config: MonitoringConfig) -> Result<(), Error> {
    println!("MONITORING PEERS");
    let peers_num = get_peers_number(config.sources).await;
    dump_peers_num(peers_num);

    Ok(())
}

async fn get_peers_number(sources: Vec<Url>) -> Vec<Result<(u64, Url), Error>> {
    let client = create_client(2);
    let mut tasks = Vec::with_capacity(sources.len());
    for source in sources {
        tasks.push(peer_num_task(client.clone(), source))
    }
    join_all(tasks).await
}

async fn peer_num_task(client: Arc<Client>, source: Url) -> Result<(u64, Url), Error> {
    let resp = client.get(source.as_str()).send().await?;
    let info_resp = resp.json::<InfoResp>().await?;
    Ok((info_resp.peers_number, source))
}

fn dump_peers_num(peers_num: Vec<Result<(u64, Url), Error>>) {
    let dump_data = format_peers_num(peers_num);
    dump_data.iter().for_each(|d| println!("{}", d));
}

fn format_peers_num(peers_num: Vec<Result<(u64, Url), Error>>) -> Vec<String> {
    let mut res: Vec<_> = peers_num
        .into_iter()
        .map(|v| v
            .map(|(num, url)| format!("Peer available under {} has {} peers", url, num))
            .unwrap_or_else(|v| v.to_string()))
        .collect();
    res.sort();
    res
}