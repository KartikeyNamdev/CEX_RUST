use actix_web::{post,get, web, App, HttpResponse, HttpServer, Responder};

use std::sync::Mutex;
mod structs;
use structs::{AppState, PublicUser, SignUpResponse, SignupBody, USER, USDBalanceResponse};


#[post("/signup")]
async fn signup(body: web::Json<SignupBody>, data : web::Data<AppState>) -> impl Responder {
    // Lock users so that other cores couldnt access it while we are checking for existing users and adding a new one
    let mut users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.username == body.username);
    match user_exists {
        Some(_) => {
            println!("User exists {:?}", user_exists);
            return HttpResponse::BadRequest().body("Username already exists");
        },
        None => {
            println!("User doesn't exist creating new user");
            let mut user_index = data.user_index.lock().unwrap();
            *user_index += 1;
            users.push(
                USER {
                    index : *user_index as u32,
                    username : body.username.clone(),
                    password : body.password.clone(),
                    usd_balance : 0.0,
                    assets : structs::Assets {
                        sol_balance : 0,
                        btc_balance : 0,
                        eth_balance : 0,
                    }
                }
            );
            println!("New User created for index {:?} successfully", user_index);
        }
    }
    HttpResponse::Ok().json( SignUpResponse{
        message : "User created successfully".to_string(),
        
        user : PublicUser {
            index : *data.user_index.lock().unwrap(),
            username : body.username.clone()
        }
    })
}

#[post("/login")]
async fn login(body: web::Json<SignupBody>, data : web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.username == body.username);

    match user_exists {
        Some(user) => {
            if user.password == body.password {
                return HttpResponse::Ok().json( SignUpResponse {
                    message : "Login successfull".to_string(),
                    user : PublicUser {
                        index : user.index,
                        username : user.username.clone()
                    }
                })
            }else {
                return HttpResponse::BadRequest().body("Incorrect password");
            }
        },
        None => {
            return HttpResponse::BadRequest().body("Username does not exist");
        }
    }
}
// GET /balance/usd where user is identified using methods like query params, headers, or cookies. For simplicity, we will use query params here.
#[get("/balance/usd")]
async fn get_usd_balance(
    query: web::Query<structs::UserQuery>,
    data: web::Data<AppState>
) -> impl Responder {
    let users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.index == query.user_id);
    match user_exists {
        Some(user) => {
            return HttpResponse::Ok().json( USDBalanceResponse{
                message : "Balance fetched successfully".to_string(),
                username : user.username.clone(),
                balance : user.usd_balance.clone()
            })
        },
        None => {
            return HttpResponse::BadRequest().body("User not found");
        }
    }
}
#[get("/balance/asset")]
async fn get_asset_balance(
    query: web::Query<structs::AssetQuery>,
    data : web::Data<AppState>
) -> impl Responder{
    let users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.index == query.user_id);
    match user_exists{
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
        },
        None => {
            return HttpResponse::BadRequest().body("User not found");
        }
    }

}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // web::Data wraps the state in an Arc internally, so this clone below
    // is just an Arc clone (cheap), not a deep copy of AppState.
    let app_state = web::Data::new(AppState {
        user_index: Mutex::new(0),
        users: Mutex::new(vec![]),
    });
    HttpServer::new(move || App::new().app_data(app_state.clone()).service(signup).service(login).service(get_usd_balance).service(get_asset_balance))
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}
