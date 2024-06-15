use std::error::Error;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

use super::{
    arbitrum_network::ArbitrumNetwork,
    config::{NetworkConfig, NetworkId},
    ethereum_network::EthereumNetwork,
    polygon_network::PolygonNetwork,
};

#[derive(Deserialize)]
struct Block {
    number: String,
}

#[derive(Deserialize)]
struct Data {
    blocks: Vec<Block>,
}

#[derive(Deserialize)]
struct ResponseData {
    data: Data,
}

pub async fn subgraph_query_block_from_timestamp(
    client: &Client,
    url: String,
    timestamp: u64,
) -> Result<u64> {
    let query: String = format!(
        r#"{{
          "query": "{{blocks(first: 1, orderBy: number, orderDirection: asc, where: {{ timestamp_gte: {}, timestamp_lt: {} }}) {{ id number timestamp }}}}"
      }}"#,
        timestamp,
        timestamp + 60
    );

    let response: reqwest::Response = client
        .post(url)
        .body(query)
        .send()
        .await
        .map_err(|err| anyhow!("couldn't retrieve data: {}", err))?;

    let parsed_data: ResponseData = response
        .json::<ResponseData>()
        .await
        .map_err(|err| anyhow!("couldn't decode response: {}", err))?;

    return parsed_data
        .data
        .blocks
        .get(0)
        .and_then(|block| block.number.parse::<u64>().ok())
        .ok_or_else(|| anyhow!("Invalid block number retrieved!"));
}

pub fn get_network_config(
    network_id: &NetworkId,
) -> Result<Box<dyn NetworkConfig>, Box<dyn Error>> {
    match network_id {
        NetworkId::ETHEREUM => Ok(Box::new(EthereumNetwork::new())),
        NetworkId::ARBITRUM => Ok(Box::new(ArbitrumNetwork::new())),
        NetworkId::POLYGON => Ok(Box::new(PolygonNetwork::new())),
        NetworkId::UNSUPPORTED_ID => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unsupported network ID",
        ))),
    }
}
