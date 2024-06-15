use std::io::Error;

use async_trait::async_trait;

#[async_trait]
pub trait NetworkConfig {
    fn get_subgraph_url() -> String;
    async fn get_block_for_timestamp(timestamp: u64) -> Result<u64, Error>;
    fn get_latest_block();
}
