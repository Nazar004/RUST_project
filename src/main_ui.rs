mod schema;
mod model;
mod controller;
mod view;

use iced::Application;
use iced::Settings;
use model::CombinedApp;

fn main() {
    if let Err(e) = CombinedApp::run(Settings::default()) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
