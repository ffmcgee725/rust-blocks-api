use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    network::{
        config::NetworkConfig, utils::get_network_config,
    },
    AppState,
};

#[derive(Debug, Deserialize, Clone)]
pub struct GetBlockFromDateRequest {
    network_id: String,
    timestamp: u64,
}

pub async fn get_block_from_date(
    // _app_state: web::Data<AppState>,
    params: web::Query<GetBlockFromDateRequest>,
) -> impl Responder {
    let network: Box<dyn NetworkConfig> = match get_network_config(&params.network_id) {
        Ok(network) => network,
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };

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
