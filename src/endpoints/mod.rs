use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    network::{config::NetworkConfig, ethereum_network::EthereumNetwork},
    AppState,
};

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

    let network: EthereumNetwork = EthereumNetwork::new();
    let block_number: Result<u64, anyhow::Error> =
        network.get_block_from_timestamp(params.timestamp).await;

    match block_number {
        Ok(block_number) => return HttpResponse::Ok().json(block_number),
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("something went wrong: {}", err));
        }
    };
}
