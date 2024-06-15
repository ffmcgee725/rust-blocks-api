use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use diesel::PgConnection;
use serde::Deserialize;

use crate::{
    network::{
        config::{NetworkConfig, NetworkId},
        utils::{
            get_block_info_from_height, get_block_info_from_timestamp, get_network_config,
            maybe_insert_block_to_db,
        },
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
    app_state: web::Data<AppState>,
    params: web::Query<GetBlockFromDateRequest>,
) -> impl Responder {
    let network: Box<dyn NetworkConfig> =
        match get_network_config(&NetworkId::from(&params.network_id)) {
            Ok(network) => network,
            Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
        };

    if &params.timestamp > &0 {
        let mut conn: std::sync::MutexGuard<PgConnection> = app_state.db.lock().unwrap();
        match get_block_info_from_timestamp(
            &mut conn,
            &params.network_id,
            &params.timestamp.try_into().unwrap(), // TODO: better solution to unwrap() here; (look into just retrieving info from subgraph as i64)
        ) {
            Some(block) => return HttpResponse::Ok().json(block.block_number),
            None => {}
        }
    }

    let block_number: Result<u64, anyhow::Error> = match params.timestamp {
        0 => network.get_latest_block().await,
        _ => network.get_block_from_timestamp(params.timestamp).await,
    };

    match block_number {
        Ok(block_number) => {
            let mut conn: std::sync::MutexGuard<PgConnection> = app_state.db.lock().unwrap();
            maybe_insert_block_to_db(
                &mut conn,
                &params.network_id,
                block_number.try_into().unwrap(),
                params.timestamp.try_into().unwrap(), // TODO: better solution to unwrap() here; (look into just retrieving info from subgraph as i64)
            );
            return HttpResponse::Ok().json(block_number);
        }
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("something went wrong: {}", err));
        }
    };
}

pub async fn get_date_from_block(
    app_state: web::Data<AppState>,
    params: web::Query<GetDateFromBlockRequest>,
) -> impl Responder {
    let network: Box<dyn NetworkConfig> =
        match get_network_config(&NetworkId::from(&params.network_id)) {
            Ok(network) => network,
            Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
        };

    if &params.block > &0 {
        let mut conn: std::sync::MutexGuard<PgConnection> = app_state.db.lock().unwrap();
        match get_block_info_from_height(
            &mut conn,
            &params.network_id,
            &params.block.try_into().unwrap(), // TODO: better solution to unwrap() here; (look into just retrieving info from subgraph as i64)
        ) {
            Some(block) => return HttpResponse::Ok().json(block.timestamp),
            None => {}
        }
    }
    let timestamp: Result<u64, anyhow::Error> = match params.block {
        0 => Ok(Utc::now().timestamp() as u64),
        _ => network.get_timestamp_from_block(params.block).await,
    };

    match timestamp {
        Ok(timestamp) => {
            let mut conn: std::sync::MutexGuard<PgConnection> = app_state.db.lock().unwrap();
            maybe_insert_block_to_db(
                &mut conn,
                &params.network_id,
                params.block.try_into().unwrap(), // TODO: better solution to unwrap() here;
                timestamp.try_into().unwrap(),
            );
            return HttpResponse::Ok().json(timestamp);
        }
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("something went wrong: {}", err));
        }
    };
}
