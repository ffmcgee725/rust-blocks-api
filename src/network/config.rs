use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait NetworkConfig {
    fn get_subgraph_url(&self) -> String;
    async fn get_block_from_timestamp(&self, timestamp: u64) -> Result<u64>;
    fn get_latest_block(&self);
}
