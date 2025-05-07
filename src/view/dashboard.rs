use iced::{
    widget::{Button, Column, Row, Space, Text},
    Alignment, Element,
};
use crate::view::combined_app::{CombinedApp, Message};

pub fn view_dashboard(app: &CombinedApp) -> Element<Message> {
    let top_bar = Row::new()
        .padding(10)
        .align_items(Alignment::Center)
        .push(Text::new(app.user_name.clone().unwrap_or_default()))
        .push(Space::with_width(iced::Length::Fill))
        .push(Button::new(Text::new("Settings")));
          //  .on_press(Message::SettingsPressed));

    let records = Column::new()
        .padding(20)
        .spacing(10)
        .push(Text::new(&app.dashboard_message))
        .push(Button::new(Text::new("Add Record"))
            .on_press(Message::ShowAddOption));

    Column::new()
        .push(top_bar)
        .push(records)
        .into()
}
