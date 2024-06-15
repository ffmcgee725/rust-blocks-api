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
    ARBITRUM,
    AVALANCHE,
    AURORA,
    BASE,
    CELO,
    ETHEREUM,
    FANTOM,
    POLYGON,
    OPTIMISM,
    RONIN,
    XDAI,
    UNSUPPORTED_ID,
}

impl NetworkId {
    pub fn from(id: &str) -> NetworkId {
        match id {
            "42161" => NetworkId::ARBITRUM,
            "43114" => NetworkId::AVALANCHE,
            "1313161554" => NetworkId::AURORA,
            "8453" => NetworkId::BASE,
            "42220" => NetworkId::CELO,
            "1" => NetworkId::ETHEREUM,
            "250" => NetworkId::FANTOM,
            "127" => NetworkId::POLYGON,
            "10" => NetworkId::OPTIMISM,
            "2020" => NetworkId::RONIN,
            "100" => NetworkId::XDAI,
            _ => NetworkId::UNSUPPORTED_ID,
        }
    }
}
