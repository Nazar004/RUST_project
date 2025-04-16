use iced::{
    widget::{Button, Column, Row, Space, Text},
    Alignment, Element, Length,
};

// Предполагаем, что ваш CombinedApp и Message определены в main_combined_app.rs.
// Чтобы не создавать циклических зависимостей, можно переэкспортировать типы в модуле view::mod.rs,
// либо указать полный путь. Здесь для простоты будем считать, что они доступны как:
// crate::view::main_combined_app::{CombinedApp, Message}

use crate::view::combined_app::{CombinedApp, Message};

pub fn view_dashboard(app: &CombinedApp) -> Element<Message> {
    // Верхняя панель: имя пользователя слева, настройки (иконка) справа
    let top_bar = Row::new()
        .padding(10)
        .align_items(Alignment::Center)
        .push(Text::new(match &app.user_name {
            Some(name) => name,
            None => "",
        }))
        .push(Space::with_width(Length::Fill))
        .push(Button::new(Text::new("⚙"))
            .on_press(Message::SettingsPressed));
    
    // Центральная часть: список записей (placeholder) и кнопка "Добавить запись"
    let records_area = Column::new()
        .padding(20)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(Text::new("Record list (placeholder)"))
        .push(Button::new(Text::new("Add Record"))
            .on_press(Message::ShowAddOption));
    
    // Объединяем верхнюю панель и основной контент
    Column::new()
        .push(top_bar)
        .push(records_area)
        .into()
}
