use crate::model::{AuthData, Transaction};
use crate::model::user::User;
use crate::model::db::DbPool;
use crate::controller::transaction_controller::load_transactions;
use crate::controller::category_controller::load_categories;

use bcrypt::{hash, verify};
use diesel::prelude::*;
use crate::schema::users::dsl::*;

pub fn attempt_login(pool: &DbPool, auth_data: &AuthData) -> Result<i32, String> {
    let mut conn = pool.get().map_err(|e| format!("DB connection error: {:?}", e))?;

    let user: User = users
        .filter(username.eq(&auth_data.username))
        .first(&mut conn)
        .map_err(|_| "User not found".to_string())?;

    if verify(&auth_data.password, &user.password).unwrap_or(false) {
        Ok(user.id)
    } else {
        Err("Invalid password".to_string())
    }
}

pub async fn handle_successful_login(pool: &DbPool, user_id: i32) -> (Vec<Transaction>, Vec<String>) {
    let txs = load_transactions(pool, user_id).unwrap_or_default();
    let cats = load_categories(pool)
        .map(|items| items.into_iter().map(|c| c.name).collect())
        .unwrap_or_default();
    (txs, cats)
}

pub async fn attempt_password_reset(
    pool: &DbPool,
    username_str: &str,
    secret_str: &str,
    new_password: &str,
) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("DB error: {:?}", e))?;
    let user: User = users
        .filter(username.eq(username_str))
        .first(&mut conn)
        .map_err(|_| "User not found".to_string())?;

    if user.secret_pass != secret_str {
        return Err("Wrong answer to secret question".into());
    }
    let hashed = hash(new_password, bcrypt::DEFAULT_COST)
        .map_err(|e| format!("Hash error: {:?}", e))?;

    diesel::update(users.filter(id.eq(user.id)))
        .set(password.eq(hashed))
        .execute(&mut conn)
        .map_err(|e| format!("Update error: {:?}", e))?;

    Ok(())
}