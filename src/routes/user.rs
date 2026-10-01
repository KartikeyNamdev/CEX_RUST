use actix_web::{post, web, HttpResponse, Responder};

use crate::structs::{AppState, PublicUser, SignUpResponse, SignupBody, USER};

#[post("/signup")]
pub async fn signup(body: web::Json<SignupBody>, data: web::Data<AppState>) -> impl Responder {
    // Lock users so that other cores couldnt access it while we are checking for existing users and adding a new one
    let mut users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.username == body.username);
    match user_exists {
        Some(_) => {
            println!("User exists {:?}", user_exists);
            return HttpResponse::BadRequest().body("Username already exists");
        }
        None => {
            println!("User doesn't exist creating new user");
            let mut user_index = data.user_index.lock().unwrap();
            *user_index += 1;
            users.push(USER {
                index: *user_index as u32,
                username: body.username.clone(),
                password: body.password.clone(),
                usd_balance: 0.0,
                assets: crate::structs::Assets {
                    sol_balance: 0,
                    btc_balance: 0,
                    eth_balance: 0,
                },
            });
            println!("New User created for index {:?} successfully", user_index);
        }
    }
    HttpResponse::Ok().json(SignUpResponse {
        message: "User created successfully".to_string(),

        user: PublicUser {
            index: *data.user_index.lock().unwrap(),
            username: body.username.clone(),
        },
    })
}

#[post("/login")]
pub async fn login(body: web::Json<SignupBody>, data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let user_exists = users.iter().find(|u| u.username == body.username);

    match user_exists {
        Some(user) => {
            if user.password == body.password {
                return HttpResponse::Ok().json(SignUpResponse {
                    message: "Login successfull".to_string(),
                    user: PublicUser {
                        index: user.index,
                        username: user.username.clone(),
                    },
                });
            } else {
                return HttpResponse::BadRequest().body("Incorrect password");
            }
        }
        None => {
            return HttpResponse::BadRequest().body("Username does not exist");
        }
    }
}
