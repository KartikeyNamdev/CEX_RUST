use actix_web::{get, post, web, HttpResponse, Responder};

use crate::structs::{self, AppState, USDBalanceResponse};

// GET /balance/usd where user is identified using methods like query params, headers, or cookies. For simplicity, we will use query params here.
#[get("/balance/usd")]
pub async fn get_usd_balance(
    query: web::Query<structs::UserQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.index == query.user_id);
    match user_exists {
        Some(user) => {
            println!("User exists {:?}", user_exists);
            return HttpResponse::Ok().json(USDBalanceResponse {
                message: "Balance fetched successfully".to_string(),
                username: user.username.clone(),
                balance: user.usd_balance.clone(),
            });
        }
        None => {
            return HttpResponse::BadRequest().body("User not found");
        }
    }
}

#[get("/balance/asset")]
pub async fn get_asset_balance(
    query: web::Query<structs::AssetQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.index == query.user_id);
    match user_exists {
        Some(user) => {
            let asset_balance = match query.asset.as_str() {
                "sol" => user.assets.sol_balance,
                "btc" => user.assets.btc_balance,
                "eth" => user.assets.eth_balance,
                _ => {
                    return HttpResponse::BadRequest().body("Invalid asset type");
                }
            };
            return HttpResponse::Ok().json(serde_json::json!({
                "message": "Asset balance fetched successfully",
                "username": user.username.clone(),
                "asset": query.asset.clone(),
                "balance": asset_balance
            }));
        }
        None => {
            return HttpResponse::BadRequest().body("User not found");
        }
    }
}

#[post("/balance/onramp")]
pub async fn on_ramp_usd(
    data: web::Data<AppState>,
    body: web::Json<structs::OnRampUSD>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let user_exists = users.iter_mut().find(|u| u.index == body.user_id);
    match user_exists {
        Some(user) => {
            user.usd_balance += body.amount;
            return HttpResponse::Ok().json(serde_json::json!({
                "message": "On-ramp successful",
                "username": user.username.clone(),
                "asset": "USD",
                "balance": user.usd_balance.clone()
            }));
        }
        None => {
            return HttpResponse::BadRequest().body("User not found");
        }
    }
}

// POST /balance/deposit -> { userId : 1, asset : "SOL", lamports : 1000000 }
#[post("/balance/deposit")]
pub async fn deposit_asset(
    body: web::Json<structs::DepositAsset>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let user_exists = users.iter_mut().find(|u| u.index == body.user_id);
    match user_exists {
        Some(user) => {
            let new_balance = match body.asset.as_str() {
                "sol" => {
                    user.assets.sol_balance += body.amount;
                    user.assets.sol_balance
                }
                "btc" => {
                    user.assets.btc_balance += body.amount;
                    user.assets.btc_balance
                }
                "eth" => {
                    user.assets.eth_balance += body.amount;
                    user.assets.eth_balance
                }
                _ => {
                    return HttpResponse::BadRequest().body("Invalid asset type");
                }
            };
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Asset Balance updated successfully",
                "username": user.username.clone(),
                "asset": body.asset.clone(),
                "balance": new_balance
            }))
        }
        None => {
            return HttpResponse::BadRequest().body("User not found");
        }
    }
}
