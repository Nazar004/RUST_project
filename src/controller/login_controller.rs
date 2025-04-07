use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use diesel::prelude::*;
use bcrypt::verify;
use crate::model::{AuthData, User};
use crate::schema::users::dsl::*;

pub fn attempt_login(
    pool: &Pool<ConnectionManager<PgConnection>>,
    auth_data: &AuthData,
) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("DB connection error: {:?}", e))?;
    let user_found = users
        .filter(username.eq(&auth_data.username))
        .first::<User>(&mut conn)
        .map_err(|_| "User not found".to_string())?;
    if verify(&auth_data.password, &user_found.password).unwrap_or(false) {
        Ok(())
    } else {
        Err("Invalid password".to_string())
    }
}
