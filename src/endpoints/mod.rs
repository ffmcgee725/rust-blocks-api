use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    network::{config::NetworkConfig, ethereum_network::EthereumNetwork},
    AppState,
};
use std::io::{Error, ErrorKind};

#[derive(Debug, Deserialize, Clone)]
pub struct GetBlockFromDateRequest {
    network_id: Option<String>, // TODO: make required!
    timestamp: u64,
}

pub async fn get_block_from_date(
    // _app_state: web::Data<AppState>,
    params: web::Query<GetBlockFromDateRequest>,
) -> impl Responder {
    // TODO: strategy pattern for network_id, for now we always fallback to EthereumNetwork

    let block_number: Result<u64, Error> = EthereumNetwork::get_block_for_timestamp(params.timestamp)
        .await
        .map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!("couldn't decode response: {}", err),
            )
        });
    match block_number {
        Ok(block_number) => return HttpResponse::Ok().json(block_number),
        Err(err) => {
            println!("something went wrong: {} -> defaulting to 0", err);
            return HttpResponse::Ok().json(0); // TODO: throw some sort of 500 error back
        }
    };
}
