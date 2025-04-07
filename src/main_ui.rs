#[macro_use]
extern crate diesel;

mod schema;
mod model;
mod controller;
mod view;

use iced::Application;
use iced::Settings;

fn main() {
    view::combined_app::CombinedApp::run(Settings::default());
}
