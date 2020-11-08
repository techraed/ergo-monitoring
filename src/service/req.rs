//! Request/response useful data and functions.

use std::time::Duration;
use std::sync::Arc;

use serde::Deserialize;
use reqwest::{Client, ClientBuilder};

#[derive(Debug, Deserialize)]
pub(super) struct InfoResp {
    #[serde(rename(deserialize = "peersCount"))]
    pub(super) peers_number: u64
}

/// Reference counted, thread safe, cloneable client, which fails requests if timeout is met
pub(super) fn create_client(timeout_secs: u64) -> Arc<Client> {
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .expect("internal error: client build failed");
    Arc::new(client)
}


