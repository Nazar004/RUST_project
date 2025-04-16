use iced::{
    widget::{TextInput, Button, Column, Row, Text},
    Alignment, Element,
};
use crate::view::combined_app::{CombinedApp, Message};

pub fn view(app: &CombinedApp) -> Element<Message> {
    let username_input = TextInput::new("User name", &app.login_username)
        .on_input(Message::LoginUsernameChanged);
    let password_input = TextInput::new("Password", &app.login_password)
        .on_input(Message::LoginPasswordChanged);
    let login_button = Button::new(Text::new("Войти"))
        .on_press(Message::LoginPressed);
    let switch_button = Button::new(Text::new("Регистрация"))
        .on_press(Message::SwitchToRegistration);

    let content = Column::new()
        .padding(20)
        .spacing(15)
        .align_items(Alignment::Center)
        .push(Text::new("Вход в систему"))
        .push(username_input)
        .push(password_input)
        .push(login_button)
        .push(Text::new(&app.login_message))
        .push(switch_button);

    Row::new()
        .align_items(Alignment::Center)
        .push(content)
        .into()
}
