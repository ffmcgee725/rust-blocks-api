use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait NetworkConfig {
    fn get_subgraph_url(&self) -> String;
    async fn get_block_from_timestamp(&self, timestamp: u64) -> Result<u64>;
    fn get_latest_block(&self);
}

#[derive(Deserialize, Debug, Clone)]
pub enum NetworkId {
    ETHEREUM,
    ARBITRUM,
    POLYGON,
    UNSUPPORTED_ID,
}

impl NetworkId {
    pub fn from(id: &str) -> NetworkId {
        match id {
            "1" => NetworkId::ETHEREUM,
            "42161" => NetworkId::ARBITRUM,
            "127" => NetworkId::POLYGON,
            _ => NetworkId::UNSUPPORTED_ID,
        }
    }
}
