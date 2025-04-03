use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use crate::model::{AuthData, NewUser, User};
use crate::schema::users;
use crate::DbPool;

pub async fn register(
    pool: web::Data<DbPool>,
    item: web::Json<AuthData>,
) -> HttpResponse {
    println!("Starting regestation: {}", item.username);
    let mut conn = pool.get().expect("Failed to get connection to DB");

    let hashed_pwd = hash(&item.password, DEFAULT_COST)
        .expect("Password hashing error");

    let new_user = NewUser {
        username: item.username.clone(),
        password: hashed_pwd,
    };

    let inserted_user = diesel::insert_into(users::table)
        .values(new_user)  // передаём значение, а не ссылку
        .get_result::<User>(&mut conn);

    match inserted_user {
        Ok(user) => {
            println!("User successfully registered: {}", user.username);
            HttpResponse::Ok().json(user)
        },
        Err(err) => {
            println!("Error during registration: {:?}", err);
            HttpResponse::InternalServerError().body("Error during registration")
        }
    }
}

pub async fn login(
    pool: web::Data<DbPool>,
    item: web::Json<AuthData>,
) -> HttpResponse {
    println!("The login process for the user has started: {}", item.username);
    let mut conn = pool.get().expect("Failed to get connection to DB");

    let user_result = users::table
        .filter(users::username.eq(&item.username))
        .first::<User>(&mut conn);

    match user_result {
        Ok(user_found) => {
            if verify(&item.password, &user_found.password).unwrap_or(false) {
                println!("The user has successfully logged in: {}", user_found.username);
                HttpResponse::Ok().body("Successful login")
            } else {
                println!("Incorrect password: {}", item.username);
                HttpResponse::Unauthorized().body("Invalid password")
            }
        },
        Err(_) => {
            println!("User not found: {}", item.username);
            HttpResponse::NotFound().body("User not found")
        }
    }
}

pub async fn protected() -> HttpResponse {
    println!("Access");
    HttpResponse::Ok().body("Access")
}