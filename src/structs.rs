use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct USER{
    pub index: u32,
    pub username: String,
    pub password: String,
    pub usd_balance: f64,
    pub assets : Assets,
}

#[derive(Debug, Clone)]
pub struct Assets{
    pub sol_balance: u64,
    pub btc_balance: u64,
    pub eth_balance: u64,
}

pub struct AppState {
    pub user_index: Mutex<u32>,
    pub users: Mutex<Vec<USER>>,
}

#[derive(Serialize, Deserialize)]
pub struct SignupBody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpResponse {
    pub message: String,
    pub user: PublicUser,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse{
    pub message: String,
    pub user: PublicUser,
}

#[derive(Serialize, Deserialize)]
pub struct USDBalanceResponse{
    pub message: String,
    pub username : String,
    pub balance: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PublicUser {
    pub index: u32,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserQuery{
    pub user_id: u32,
}
#[derive(Serialize, Deserialize)]
pub struct AssetQuery{
    pub user_id: u32,
    pub asset: String,
}
// POST /signup
// POST /singin
// GET /balance/usd -> Native currency
// GET /balance?asset=sol -> Specific asset info
// POST /balance/onramp -> { userId : 1, amount : 200 }
// POST /balance/deposit -> { userId : 1, asset : “SOL”, lamports : 1000000 }
// ———
// POST /order { market : “SOL_USDC”, type : “buy/sell”, qty: 1, price? : 75.22, type: “limit” }
