use actix_web::{web, App, HttpServer};

use std::sync::Mutex;

mod auth;
mod routes;
mod structs;

use routes::balance::{deposit_asset, get_asset_balance, get_usd_balance, on_ramp_usd};
use routes::user::{login, signup};
use structs::AppState;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // web::Data wraps the state in an Arc internally, so this clone below
    // is just an Arc clone (cheap), not a deep copy of AppState.
    let app_state = web::Data::new(AppState {
        user_index: Mutex::new(0),
        users: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(signup)
            .service(login)
            .service(get_usd_balance)
            .service(get_asset_balance)
            .service(on_ramp_usd)
            .service(deposit_asset)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
