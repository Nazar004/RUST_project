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
        .push(IcedText::new("Register"))
        .push(
            TextInput::new("Username", &app.reg_username)
                .on_input(Message::RegUsernameChanged),
        )
        .push(
            TextInput::new("Password", &app.reg_password)
                .on_input(Message::RegPasswordChanged)
                .password(), 
        )
        .push(
            TextInput::new("Confirm Password", &app.reg_confirm)
                .on_input(Message::RegConfirmChanged)
                .password(),
        )
        .push(
        TextInput::new("Your first university", &app.secret_pass)
            .on_input(Message::SecretPassChanged)
        )
        .push(Button::new(IcedText::new("Register")).on_press(Message::RegisterPressed))
        .push(IcedText::new(&app.reg_message))
        .push(Button::new(IcedText::new("Login")).on_press(Message::SwitchToLogin))
        .into()
}
