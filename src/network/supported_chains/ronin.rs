use anyhow::Result;
use async_trait::async_trait;

use crate::network::{
    config::NetworkConfig,
    utils::{
        subgraph_query_block_from_timestamp, subgraph_query_latest_block,
        subgraph_query_timestamp_from_block,
    },
};

pub struct RoninNetwork {
    client: reqwest::Client,
}

impl RoninNetwork {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NetworkConfig for RoninNetwork {
    fn get_subgraph_url(&self) -> String {
        return String::from(
            "https://thegraph-v2.roninchain.com/subgraphs/name/axieinfinity/ronin-blocks",
        );
    }

    async fn get_timestamp_from_block(&self, block: i64) -> Result<i64> {
        return subgraph_query_timestamp_from_block(&self.client, self.get_subgraph_url(), block)
            .await;
    }

    async fn get_block_from_timestamp(&self, timestamp: i64) -> Result<i64> {
        return subgraph_query_block_from_timestamp(
            &self.client,
            self.get_subgraph_url(),
            timestamp,
        )
        .await;
    }

    async fn get_latest_block(&self) -> Result<i64> {
        return subgraph_query_latest_block(&self.client, self.get_subgraph_url()).await;
    }
}
