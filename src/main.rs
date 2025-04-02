#[macro_use]
extern crate diesel;

pub mod schema;
pub mod model {
    include!("model/model.rs");
}
pub mod controller {
    include!("controller/controller.rs");
}
pub mod view {
    include!("view/view.rs");
}

use std::io::{self, Write};
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use actix_web::web;
use model::AuthData;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Функция для чтения ввода пользователя из консоли
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Переменная DATABASE_URL не установлена");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Не удалось создать пул соединений");

    println!("Выберите действие:");
    println!("1) Регистрация");
    println!("2) Логин");
    println!("3) Выход");

    let choice = prompt("Ваш выбор: ");

    match choice.as_str() {
        "1" => {
            println!("--- Regestation ---");
            let username = prompt("Введите имя пользователя: ");
            let password = prompt("Введите пароль: ");
            let auth_data = AuthData { username, password };
            let json_data = web::Json(auth_data);
            let response = controller::register(web::Data::new(pool), json_data).await;
            println!("Ответ: {:?}", response);
        },
        "2" => {
            println!("--- Login ---");
            let username = prompt("Введите имя пользователя: ");
            let password = prompt("Введите пароль: ");
            let auth_data = AuthData { username, password };
            let json_data = web::Json(auth_data);
            let response = controller::login(web::Data::new(pool), json_data).await;
            println!("Ответ: {:?}", response);
        },
        "3" => {
            println!("Exit.");
        },
        _ => {
            println!("***");
        }
    }

    Ok(())
}
