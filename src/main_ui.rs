#[macro_use]
extern crate diesel;

mod schema;
mod model;
mod controller;
mod view;

use std::process::Command;
use std::env;
use iced::Application;
use iced::Settings;

fn main() {
    // Путь к скрипту (в той же папке, что исполняемый файл)
    let script_name = if cfg!(target_os = "windows") {
        "create_all.bat"
    } else {
        "create_all.sh"
    };

    // Получаем текущую директорию (где запускается исполняемый файл)
    let current_dir = env::current_dir().expect("Не удалось получить текущую директорию");

    // Создаём полный путь к скрипту
    let script_dir = current_dir.join("src"); // путь к папке, где .bat и .sql
    let script_path = script_dir.join(script_name);
    
    // Проверка
    if !script_path.exists() {
        eprintln!("❌ Скрипт не найден по пути: {}", script_path.display());
        return;
    }
    
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(&script_dir) // 👈 установить рабочую директорию
            .args(&["/C", "call", script_path.to_str().unwrap()])
            .status()
    } else {
        Command::new("sh")
            .current_dir(&script_dir)
            .arg(script_path.to_str().unwrap())
            .status()
    };
    

    // Проверка результата выполнения
    match status {
        Ok(s) if s.success() => {
            println!("Скрипт выполнен успешно.");
        }
        Ok(s) => {
            eprintln!("Скрипт завершился с кодом: {}", s);
        }
        Err(e) => {
            eprintln!("Ошибка при запуске скрипта: {}", e);
        }
    }

    // Запуск приложения
    view::combined_app::CombinedApp::run(Settings::default()).unwrap();
}
