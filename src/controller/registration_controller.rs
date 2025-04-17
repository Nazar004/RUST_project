use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use diesel::prelude::*;
use bcrypt::{hash, DEFAULT_COST};
use crate::model::{AuthData, NewUser};

pub fn attempt_register(
    pool: &Pool<ConnectionManager<PgConnection>>,
    auth_data: &AuthData,
) -> Result<(), String> {
    let mut conn = pool.get().map_err(|e| format!("DB connection error: {:?}", e))?;
    let hashed = hash(&auth_data.password, DEFAULT_COST)
        .map_err(|e| format!("Password error: {:?}", e))?;
    let new_user = NewUser {
        username: auth_data.username.clone(),
        password: hashed,
    };
    diesel::insert_into(crate::schema::users::table)
        .values(new_user)
        .execute(&mut conn)
        .map_err(|e| format!("Registration error: {:?}", e))?;
    Ok(())
}