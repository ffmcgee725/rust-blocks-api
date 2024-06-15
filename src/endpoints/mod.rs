use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;

use crate::{
    network::{
        config::{NetworkConfig, NetworkId},
        utils::get_network_config,
    },
    AppState,
};

#[derive(Debug, Deserialize, Clone)]
pub struct GetBlockFromDateRequest {
    network_id: String,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetDateFromBlockRequest {
    network_id: String,
    block: u64,
}

pub async fn get_block_from_date(
    // _app_state: web::Data<AppState>,
    params: web::Query<GetBlockFromDateRequest>,
) -> impl Responder {
    let network: Box<dyn NetworkConfig> =
        match get_network_config(&NetworkId::from(&params.network_id)) {
            Ok(network) => network,
            Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
        };

    let block_number: Result<u64, anyhow::Error> = match params.timestamp {
        0 => network.get_latest_block().await,
        _ => network.get_block_from_timestamp(params.timestamp).await,
    };

    match block_number {
        Ok(block_number) => return HttpResponse::Ok().json(block_number),
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("something went wrong: {}", err));
        }
    };
}

pub async fn get_date_from_block(
    // _app_state: web::Data<AppState>,
    params: web::Query<GetDateFromBlockRequest>,
) -> impl Responder {
    let network: Box<dyn NetworkConfig> =
        match get_network_config(&NetworkId::from(&params.network_id)) {
            Ok(network) => network,
            Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
        };

    let timestamp: Result<u64, anyhow::Error> = match params.block {
        0 => Ok(Utc::now().timestamp() as u64),
        _ => network.get_timestamp_from_block(params.block).await,
    };

    match timestamp {
        Ok(block_number) => return HttpResponse::Ok().json(block_number),
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("something went wrong: {}", err));
        }
    };
}
