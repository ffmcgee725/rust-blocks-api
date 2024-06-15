use std::error::Error;

use anyhow::{anyhow, Result};
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;

use serde::Deserialize;

use super::{
    config::{NetworkConfig, NetworkId},
    supported_chains::{
        arbitrum::ArbitrumNetwork, aurora::AuroraNetwork, avalanche::AvalancheNetwork,
        base::BaseNetwork, celo::CeloNetwork, ethereum::EthereumNetwork, fantom::FantomNetwork,
        optimism::OptimismNetwork, polygon::PolygonNetwork, ronin::RoninNetwork, xdai::XDaiNetwork,
    },
};

#[derive(Deserialize)]
struct Block {
    timestamp: String,
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
          "query": "{{blocks(first: 1, orderBy: number, orderDirection: asc, where: {{ timestamp_gte: {}, timestamp_lt: {} }}) {{ number timestamp }}}}"
      }}"#,
        timestamp,
        timestamp + 60
    );

    let parsed_data: ResponseData = subgraph_parse_response_data(client, url, query).await?;

    return parsed_data
        .data
        .blocks
        .get(0)
        .and_then(|block| block.number.parse::<u64>().ok())
        .ok_or_else(|| anyhow!("Invalid block number retrieved!"));
}

pub async fn subgraph_query_timestamp_from_block(
    client: &Client,
    url: String,
    block: u64,
) -> Result<u64> {
    let query: String = format!(
        r#"{{
          "query": "{{blocks(first: 1, orderBy: number, orderDirection: asc, where: {{ number_gte: {}, number_lt: {} }}) {{ number timestamp }}}}"
      }}"#,
        block,
        block + 5
    );

    let parsed_data: ResponseData = subgraph_parse_response_data(client, url, query).await?;

    return parsed_data
        .data
        .blocks
        .get(0)
        .and_then(|block| block.timestamp.parse::<u64>().ok())
        .ok_or_else(|| anyhow!("Invalid timestamp retrieved!"));
}

pub async fn subgraph_query_latest_block(client: &Client, url: String) -> Result<u64> {
    let query: String = format!(
        r#"{{
              "query": "{{blocks(first: 1, orderBy: number, orderDirection: desc) {{ number timestamp }}}}"
          }}"#
    );

    let parsed_data: ResponseData = subgraph_parse_response_data(client, url, query).await?;

    return parsed_data
        .data
        .blocks
        .get(0)
        .and_then(|block| block.number.parse::<u64>().ok())
        .ok_or_else(|| anyhow!("Invalid block number retrieved!"));
}

async fn subgraph_parse_response_data(
    client: &Client,
    url: String,
    query: String,
) -> Result<ResponseData> {
    let response: reqwest::Response = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(query)
        .send()
        .await
        .map_err(|err| anyhow!("couldn't retrieve data: {}", err))?;

    let parsed_data: ResponseData = response
        .json::<ResponseData>()
        .await
        .map_err(|err| anyhow!("couldn't decode response: {}", err))?;

    return Ok(parsed_data);
}

pub fn get_network_config(
    network_id: &NetworkId,
) -> Result<Box<dyn NetworkConfig>, Box<dyn Error>> {
    match network_id {
        NetworkId::ARBITRUM => Ok(Box::new(ArbitrumNetwork::new())),
        NetworkId::AVALANCHE => Ok(Box::new(AvalancheNetwork::new())),
        NetworkId::AURORA => Ok(Box::new(AuroraNetwork::new())),
        NetworkId::BASE => Ok(Box::new(BaseNetwork::new())),
        NetworkId::CELO => Ok(Box::new(CeloNetwork::new())),
        NetworkId::ETHEREUM => Ok(Box::new(EthereumNetwork::new())),
        NetworkId::FANTOM => Ok(Box::new(FantomNetwork::new())),
        NetworkId::POLYGON => Ok(Box::new(PolygonNetwork::new())),
        NetworkId::OPTIMISM => Ok(Box::new(OptimismNetwork::new())),
        NetworkId::RONIN => Ok(Box::new(RoninNetwork::new())),
        NetworkId::XDAI => Ok(Box::new(XDaiNetwork::new())),
        NetworkId::UNSUPPORTED_ID => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unsupported network ID",
        ))),
    }
}
