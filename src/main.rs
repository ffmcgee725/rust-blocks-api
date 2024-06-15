pub mod database;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use database::connection::establish_connection;
use diesel;
use diesel::pg::PgConnection;
use std::sync::Mutex;

pub struct AppState {
    db: Mutex<PgConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: PgConnection = establish_connection();

    let data: web::Data<AppState> = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            // .route("/token", web::post().to(get_token_price))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
