use iced::{
    widget::{Button, Column, Text as IcedText, TextInput},
    Alignment, Element,
};

use crate::model::{CombinedApp, Message};

pub fn render(app: &CombinedApp) -> Element<Message> {
    Column::new()
        .padding(20)
        .spacing(15)
        .align_items(Alignment::Center)
        .push(IcedText::new("Login"))
        .push(
            TextInput::new("Username", &app.login_username)
                .on_input(Message::LoginUsernameChanged),
        )
        .push(
            TextInput::new("Password", &app.login_password)
                .on_input(Message::LoginPasswordChanged)
                .password(),
        )
        .push(Button::new(IcedText::new("Login")).on_press(Message::LoginPressed))
        .push(IcedText::new(&app.login_message))
        .push(Button::new(IcedText::new("Register")).on_press(Message::SwitchToRegistration))
        .push(Button::new(IcedText::new("Exit")).on_press(Message::ExitPressed))
        .push(Button::new(IcedText::new("Forgot password?"))
        .on_press(Message::RequestPasswordReset),)
        .into()
}
