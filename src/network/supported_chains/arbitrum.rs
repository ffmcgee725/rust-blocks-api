use anyhow::Result;
use async_trait::async_trait;

use crate::network::{config::NetworkConfig, utils::subgraph_query_block_from_timestamp};

pub struct ArbitrumNetwork {
    client: reqwest::Client,
}

impl ArbitrumNetwork {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NetworkConfig for ArbitrumNetwork {
    fn get_subgraph_url(&self) -> String {
        return String::from(
            "https://api.thegraph.com/subgraphs/name/ianlapham/arbitrum-one-blocks",
        );
    }

    async fn get_block_from_timestamp(&self, timestamp: u64) -> Result<u64> {
        return subgraph_query_block_from_timestamp(
            &self.client,
            self.get_subgraph_url(),
            timestamp,
        )
        .await;
    }

    fn get_latest_block(&self) {
        todo!()
    }
}
