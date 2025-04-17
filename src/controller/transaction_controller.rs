// src/controller/transaction_controller.rs

use std::env;
use postgres::{Client, NoTls};
use crate::model::Transaction;

/// Загрузить все транзакции пользователя
pub fn load_transactions(uid: i32) -> Result<Vec<Transaction>, String> {
    // Получаем URL из .env (или из окружения)
    let url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set".to_string())?;
    // Открываем соединение
    let mut client = Client::connect(&url, NoTls)
        .map_err(|e| format!("DB connection error: {:?}", e))?;

    // Готовим и выполняем запрос
    let rows = client.query(
        "
        SELECT
            tran_id,
            tran_type,
            user_id,
            tran_source,
            date,
            tran_amount,
            tran_comment
        FROM transactions
        WHERE user_id = $1
        ORDER BY date DESC
        ",
        &[&uid],
    )
    .map_err(|e| format!("Load transactions error: {:?}", e))?;

    // Мапим каждую строку в нашу структуру
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

/// Удалить транзакцию по её tran_id
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
