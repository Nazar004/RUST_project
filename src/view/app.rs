use iced::{Application, executor, Command, Element, Theme};

use crate::controller::app_controller;
use crate::model::{CombinedApp, Screen, Message};
use crate::view::{login_view, register_view, dashboard_view};

use super::reset_password_view;

impl Application for CombinedApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (CombinedApp::default(), Command::none())
    }

    fn title(&self) -> String {
        "Finance Dashboard".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        app_controller::update(self, message)
    }

    fn view(&self) -> Element<Message> {
        match &self.current_screen {
            Screen::Login => login_view::render(self),
            Screen::Registration => register_view::render(self),
            Screen::Dashboard(mode) => dashboard_view::render(self, mode),
            Screen::ResetPassword => reset_password_view::render(self),
        }
    }
}
