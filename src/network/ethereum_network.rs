use actix_web::cookie::time::format_description::parse;
use async_trait::async_trait;
use serde::Deserialize;

use super::config::NetworkConfig;
use std::io::{Error, ErrorKind};

// TODO move these structs elsewhere ?
#[derive(Deserialize, Debug)]
struct Block {
    id: String,
    number: String,
    timestamp: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    blocks: Vec<Block>,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    data: Data,
}
pub struct EthereumNetwork {}

#[async_trait]
impl NetworkConfig for EthereumNetwork {
    fn get_subgraph_url() -> String {
        return String::from("https://api.thegraph.com/subgraphs/name/snowfork/ethereum-blocks");
    }

    async fn get_block_for_timestamp(timestamp: u64) -> Result<u64, Error> {
        let url: String = EthereumNetwork::get_subgraph_url();

        // TODO: create a single client, that's part of the struct EthereumNetwork;
        let client = reqwest::Client::new();
        let query = format!(
            r#"{{
                "query": "{{blocks(first: 1, orderBy: number, orderDirection: asc, where: {{ timestamp_gte: {}, timestamp_lt: {} }}) {{ id number timestamp }}}}"
            }}"#,
            timestamp,
            timestamp + 60
        );
        let response: reqwest::Response =
            client.post(url).body(query).send().await.map_err(|err| {
                Error::new(ErrorKind::Other, format!("couldn't retrieve data: {}", err))
            })?;

        let parsed_data: ResponseData = response.json::<ResponseData>().await.map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!("couldn't decode response: {}", err),
            )
        })?;

        // TODO: clean this up
        match parsed_data.data.blocks[0].number.parse::<u64>() {
            Ok(parsed_value) => return Ok(parsed_value),
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Invalid block number retrieved!"),
                ))
            }
        };
    }

    fn get_latest_block() {
        todo!()
    }
}
