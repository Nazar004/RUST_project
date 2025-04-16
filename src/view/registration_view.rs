use iced::{
    widget::{TextInput, Button, Column, Row, Text},
    Alignment, Element,
};
use crate::view::combined_app::{CombinedApp, Message};

pub fn view(app: &CombinedApp) -> Element<Message> {
    let username_input = TextInput::new("Имя пользователя", &app.reg_username)
        .on_input(Message::RegUsernameChanged);
    let password_input = TextInput::new("Пароль", &app.reg_password)
        .on_input(Message::RegPasswordChanged);
    let confirm_input = TextInput::new("Подтвердите пароль", &app.reg_confirm)
        .on_input(Message::RegConfirmChanged);
    let register_button = Button::new(Text::new("Зарегистрироваться"))
        .on_press(Message::RegisterPressed);
    let switch_button = Button::new(Text::new("Вход"))
        .on_press(Message::SwitchToLogin);

    let content = Column::new()
        .padding(20)
        .spacing(15)
        .align_items(Alignment::Center)
        .push(Text::new("Регистрация"))
        .push(username_input)
        .push(password_input)
        .push(confirm_input)
        .push(register_button)
        .push(Text::new(&app.reg_message))
        .push(switch_button);

    Row::new()
        .align_items(Alignment::Center)
        .push(content)
        .into()
}
