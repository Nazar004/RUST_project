
use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Простой счетчик на Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(format!("Значение: {}", self.value)))
            .push(
                Button::new(&mut self.increment_button, Text::new("Увеличить"))
                    .on_press(Message::Increment),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Уменьшить"))
                    .on_press(Message::Decrement),
            );

        content.into()
    }
}
