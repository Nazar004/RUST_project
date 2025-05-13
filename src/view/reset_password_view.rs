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
        .push(IcedText::new("Reset Password"))
        .push(
            TextInput::new("Username", &app.login_username)
                .on_input(Message::LoginUsernameChanged)
        )
        .push(
            TextInput::new("Your first university", &app.secret_pass)
                .on_input(Message::SecretPassChanged)
        )
        .push(
            TextInput::new("New password", &app.new_password)
                .on_input(Message::NewPasswordChanged)
                .password()
        )
        .push(
            TextInput::new("Confirm new password", &app.confirm_new_password)
                .on_input(Message::ConfirmNewPasswordChanged)
                .password()
        )
        .push(
            Button::new(IcedText::new("Submit"))
                .on_press(Message::SubmitPasswordReset),
        )
        .push(
            IcedText::new(&app.reg_message) // здесь выводим ошибки/успех
        )
        .push(
            Button::new(IcedText::new("Back to Login"))
                .on_press(Message::SwitchToLogin),
        )
        .into()
}
