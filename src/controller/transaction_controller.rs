
use std::env;
use postgres::{Client, NoTls};
use crate::model::Transaction;

// для Diesel-вставок:
use diesel::prelude::*;
use diesel::insert_into;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use crate::schema::transactions::dsl::*;

/// Загрузить все транзакции пользователя
pub fn load_transactions(uid: i32) -> Result<Vec<Transaction>, String> {
    let url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set".to_string())?;
    let mut client = Client::connect(&url, NoTls)
        .map_err(|e| format!("DB connection error: {:?}", e))?;

    let rows = client.query(
        "
        SELECT tran_id, tran_type, user_id, tran_source, date, tran_amount, tran_comment
        FROM transactions
        WHERE user_id = $1
        ORDER BY date DESC
        ",
        &[&uid],
    )
    .map_err(|e| format!("Load transactions error: {:?}", e))?;

    let result = rows.into_iter().map(|row| Transaction {
        tran_id:      row.get("tran_id"),
        tran_type:    row.get("tran_type"),
        user_id:      row.get("user_id"),
        tran_source:  row.get("tran_source"),
        date:         row.get("date"),
        tran_amount:  row.get("tran_amount"),
        tran_comment: row.get("tran_comment"),
    })
    .collect();

    Ok(result)
}

/// Удалить транзакцию
pub fn delete_transaction(tid: i32) -> Result<u64, String> {
    let url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set".to_string())?;
    let mut client = Client::connect(&url, NoTls)
        .map_err(|e| format!("DB connection error: {:?}", e))?;

    let count = client.execute(
        "DELETE FROM transactions WHERE tran_id = $1",
        &[&tid],
    )
    .map_err(|e| format!("Delete transaction error: {:?}", e))?;

    Ok(count)
}

/// Вставить расход
pub fn add_expense(
    pool: &Pool<ConnectionManager<PgConnection>>,
    uid: i32,
    source_str: &str,
    date_str: &str,
    amount_val: f64,
) -> Result<usize, diesel::result::Error> {
    let mut conn = pool.get().expect("DB pool error");
    insert_into(transactions)
        .values((
            tran_type.eq("Expense"),
            user_id.eq(uid),
            tran_source.eq(source_str),
            date.eq(date_str),
            tran_amount.eq(amount_val),
        ))
        .execute(&mut conn)
}

/// Вставить доход
pub fn add_income(
    pool: &Pool<ConnectionManager<PgConnection>>,
    uid: i32,
    source_str: &str,
    date_str: &str,
    amount_val: f64,
) -> Result<usize, diesel::result::Error> {
    let mut conn = pool.get().expect("DB pool error");
    insert_into(transactions)
        .values((
            tran_type.eq("Income"),
            user_id.eq(uid),
            tran_source.eq(source_str),
            date.eq(date_str),
            tran_amount.eq(amount_val),
        ))
        .execute(&mut conn)
}
