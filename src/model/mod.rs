use serde::{Deserialize, Serialize};
use diesel::Queryable;
use diesel::Insertable;
use crate::schema::users;
use crate::schema::transactions;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}


// Новая структура — транзакция

// src/model/mod.rs
// src/model/mod.rs


#[derive(Debug, Queryable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub tran_type:   String,
    pub user_id:     i32,
    pub tran_id:     i32,
    pub tran_source: String,
    pub date:        String,
    pub tran_amount: f64,              
    pub tran_comment: Option<String>,
}



