// src/controller/transaction_controller.rs
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use diesel::result::Error as DieselError;
use crate::schema::transactions::dsl::*;
use crate::model::{Transaction, NewTransaction};

/// Загрузить все транзакции пользователя
pub fn load_transactions(
    pool: &Pool<ConnectionManager<PgConnection>>,
    uid: i32
) -> Result<Vec<Transaction>, DieselError> {
    let mut conn = pool.get().map_err(|_| DieselError::NotFound)?;
    transactions
        .filter(user_id.eq(uid))
        .order(date.desc())
        .load::<Transaction>(&mut conn)
}

/// Вставить расход
pub fn add_expense(
    pool: &Pool<ConnectionManager<PgConnection>>,
    uid: i32,
    source_str: &str,
    date_str: &str,
    amount_val: f64,
    tag_id_val: Option<i32>,
) -> Result<(), DieselError> {
    let mut conn = pool.get().map_err(|_| DieselError::NotFound)?;


    let new_tx = NewTransaction {
        tran_type: "Expense",
        user_id: uid,
        tran_source: source_str,
        date: date_str,
        tran_amount: amount_val,
        tag_id: tag_id_val,
        tran_comment: None,
    };

    diesel::insert_into(transactions)
        .values(&new_tx)
        .execute(&mut conn)?;

    Ok(())
}


/// Вставить доход
pub fn add_income(
    pool: &Pool<ConnectionManager<PgConnection>>,
    uid: i32,
    source_str: &str,
    date_str: &str,
    amount_val: f64,
) -> Result<usize, DieselError> {
    let mut conn = pool.get().map_err(|_| DieselError::NotFound)?;
    diesel::insert_into(transactions)
        .values((
            tran_type.eq("Income"),
            user_id.eq(uid),
            tran_source.eq(source_str),
            date.eq(date_str),
            tran_amount.eq(amount_val),
        ))
        .execute(&mut conn)
}