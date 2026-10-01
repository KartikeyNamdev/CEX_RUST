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
    pub token: String,
    pub user: PublicUser,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse{
    pub message: String,
    pub token: String,
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
#[derive(Serialize,Deserialize)]
pub struct OnRampUSD{
    pub user_id: u32,
    pub amount: f64,
}
#[derive(Serialize,Deserialize)]
pub struct DepositAsset{
    pub user_id : u32,
    pub asset : String,
    pub amount : u64, // in lamports for SOL, satoshis for BTC, wei for ETH
}

pub struct DepositAssetResponse{
    pub message : String,
    pub username : String,
    pub asset : String,
    pub balance : f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // "subject" — the user's index. Standard JWT convention for "who this token is about"
    pub exp: usize,  // expiry, as a Unix timestamp — required by the jsonwebtoken crate
}

pub struct AuthedUser {
    pub username : String,
}