// model/transaction.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::transactions;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub tran_id: i32,
    pub tran_type: String,
    pub user_id: i32,
    pub tran_source: String,
    pub date: String,
    pub tran_amount: f64,
    pub tran_comment: Option<String>,
    pub tag_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub tran_type: &'a str,
    pub user_id: i32,
    pub tran_source: &'a str,
    pub date: &'a str,
    pub tran_amount: f64,
    pub tag_id: Option<i32>,
    pub tran_comment: Option<&'a str>,
}
