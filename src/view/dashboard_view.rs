use iced::{
    widget::{Button, Canvas, Column, PickList, Row, Scrollable, Space, Text as IcedText, TextInput},
    Alignment, Color, Element, Length, Point, Rectangle, mouse::Cursor, Renderer, Theme,Background,
};
use iced::widget::canvas::{Frame, Path, Program, Geometry, Text as CanvasText};
use iced::widget::canvas::path::Arc as CanvasArc;
use iced::widget::Container;



use std::collections::HashMap;
// use std::f32::consts::PI;

use crate::model::{CombinedApp, DashboardViewMode, Message};
use crate::model::state::SortType;
// Добавить в dashboard_view.rs






struct BlackBackground;

impl iced::widget::container::StyleSheet for BlackBackground {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(Color::BLACK)),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}


struct BodyBackground;

impl iced::widget::container::StyleSheet for BodyBackground {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.686, 0.933, 0.933))), // светло-серый
            text_color: None,
            ..Default::default()
        }
    }
}
struct TransactionListBackground;

impl iced::widget::container::StyleSheet for TransactionListBackground {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.9, 0.9))),
            border_width: 2.0,
            border_color: Color::BLACK,
            text_color: None,
            ..Default::default()
        }
    }
}



pub fn render<'a>(
    app: &'a CombinedApp,
    mode: &'a DashboardViewMode,
) -> Element<'a, Message> {
    match mode {
        DashboardViewMode::Main => render_dashboard_main(&app),
        DashboardViewMode::AddExpense => render_add_expense(&app),
        DashboardViewMode::AddIncome => render_add_income(&app),
    }
}

fn render_dashboard_main(app: &CombinedApp) -> Element<Message> {
    let mut totals: HashMap<String, f32> = HashMap::new();
    for cat in &app.categories {
        totals.insert(cat.clone(), 0.0);
    }
    for tx in &app.transactions {
        if tx.tran_type == "Expense" {
            if let Some(tag_id) = tx.tag_id {
                if let Some(name) = app.categories.get((tag_id - 1) as usize) {
                    *totals.get_mut(name).unwrap() += tx.tran_amount as f32;
                }
            }
        }
    }

    let chart_data: Vec<(String, f32)> = totals.into_iter().filter(|(_, sum)| sum > &0.0).collect();

    struct PieChart {
        data: Vec<(String, f32)>,
    }
    impl<Message> Program<Message> for PieChart {
        type State = ();
        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            _theme: &Theme,
            bounds: Rectangle,
            _cursor: Cursor,
        ) -> Vec<Geometry> {
            let mut frame = Frame::new(renderer, bounds.size());
            let center = frame.center();
            let outer_radius = center.x.min(center.y) * 0.8;
            let inner_radius = outer_radius * 0.5;
            let total: f32 = self.data.iter().map(|(_, v)| *v).sum();

            let mut start_angle = 0.0_f32;
            for (i, (label, value)) in self.data.iter().enumerate() {
                let pct = if total > 0.0 { value / total } else { 0.0 };
                let sweep = pct * 2.0 * std::f32::consts::PI;
                let end_angle = start_angle + sweep;

                // let sx = center.x + outer_radius * start_angle.cos();
                // let sy = center.y + outer_radius * start_angle.sin();

                // let path = Path::new(|p| {
                //     p.move_to(center);
                //     p.line_to(Point::new(sx, sy));
                //     p.arc(CanvasArc {
                //         center,
                //         radius: outer_radius,
                //         start_angle,
                //         end_angle,
                //     });
                //     p.close();
                // });
                let path = Path::new(|p| {
                    // Внешняя дуга — от начала до конца
                    p.move_to(Point {
                        x: center.x + outer_radius * start_angle.cos(),
                        y: center.y + outer_radius * start_angle.sin(),
                    });
                      // Линия от внешней к внутренней дуге
                    p.line_to(Point {
                        x: center.x + inner_radius * end_angle.cos(),
                        y: center.y + inner_radius * end_angle.sin(),
                    });
                    p.arc(CanvasArc {
                        center,
                        radius: outer_radius,
                        start_angle,
                        end_angle,
                    });

                  

                    // Внутренняя дуга — от конца к началу, в обратную сторону
                    p.arc(CanvasArc {
                        center,
                        radius: inner_radius,
                        start_angle: end_angle,
                        end_angle: start_angle,
                    });

                    p.close(); // замыкаем путь
                });


                let color = match i % 8 {
                    0 => Color::from_rgb(0.8, 0.1, 0.4),
                    1 => Color::from_rgb(0.1, 0.8, 0.4),
                    2 => Color::from_rgb(0.4, 0.4, 1.0),
                    3 => Color::from_rgb(1.0, 0.6, 0.2),
                    4 => Color::from_rgb(0.6, 0.2, 1.0),
                    5 => Color::from_rgb(0.2, 1.0, 0.8),
                    6 => Color::from_rgb(0.8, 0.8, 0.2),
                    _ => Color::from_rgb(0.3, 0.3, 0.3),
                };

                frame.fill(&path, color);

                // Подписи
                let mid_angle = start_angle + sweep / 2.0;
                let tx = center.x + outer_radius * 0.65 * mid_angle.cos();
                let ty = center.y + outer_radius * 0.65 * mid_angle.sin();
                frame.fill_text(CanvasText {
                    content: format!("{} ({:.0}%)", label, pct * 100.0),
                    position: Point::new(tx, ty),
                    color: Color::BLACK,
                    size: 14.0,
                    ..Default::default()
                });

                start_angle = end_angle;
            }

            // Очистим центр: белый круг
            let clear_center = Path::circle(center, inner_radius);
            frame.fill(&clear_center, Color::from_rgb(0.686, 0.933, 0.933));

            vec![frame.into_geometry()]
        }

        // fn draw(&self, _state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        //     let mut frame = Frame::new(renderer, bounds.size());
        //     let center = frame.center();
        //     let radius = center.x.min(center.y) * 0.8;
        //     let total: f32 = self.data.iter().map(|(_, v)| *v).sum();

        //     let mut start_angle = 0.0_f32;
        //     for (i, (label, value)) in self.data.iter().enumerate() {
        //         let pct = if total > 0.0 { value / total } else { 0.0 };
        //         let sweep = pct * 2.0 * PI;
        //         let end_angle = start_angle + sweep;

        //     let path = Path::new(|p| {
        //         p.move_to(Point::new(
        //             center.x + radius * start_angle.cos(),
        //             center.y + radius * start_angle.sin(),      
        //         ));
        //         p.arc(CanvasArc {
        //             center,
        //             radius,
        //             start_angle,
        //             end_angle,
        //         });
        //         p.arc(CanvasArc {
        //             center,
        //             radius: radius * 0.5, // внутренний радиус — "дырка"
        //             start_angle: end_angle,
        //             end_angle: start_angle,
        //         });
        //         p.close();
        //     });

            
        //         let color = match i % 5 {
        //             0 => Color::from_rgb(0.9, 0.3, 0.3),
        //             1 => Color::from_rgb(0.3, 0.9, 0.3),
        //             2 => Color::from_rgb(0.3, 0.3, 0.9),
        //             3 => Color::from_rgb(0.9, 0.9, 0.3),
        //             _ => Color::from_rgb(0.7, 0.3, 0.9),
        //         };
        //         frame.fill(&path, color);
        //         let mid_angle = start_angle + sweep / 2.0;
        //         let tx = center.x + radius * 0.6 * mid_angle.cos();
        //         let ty = center.y + radius * 0.6 * mid_angle.sin();
        //         frame.fill_text(CanvasText {
        //             content: format!("{} ({:.0}%)", label, pct * 100.0),
        //             position: Point::new(tx, ty),
        //             color: Color::BLACK,
        //             size: 14.0,
        //             ..CanvasText::default()
        //         });
        //         start_angle = end_angle;
        //     }
        //     vec![frame.into_geometry()]
        // }
    }

    let pie = Canvas::new(PieChart { data: chart_data })
        .width(Length::Fixed(250.0))
        .height(Length::Fixed(250.0));

    let sort_picker = PickList::new(
        &SortType::ALL[..],
        Some(app.sort_type.clone()),
        Message::SortTypeChanged,
    );

    let top_controls = Row::new()
        .padding(10)
        .spacing(20)
        .push(IcedText::new("Sorting:"))
        .push(sort_picker);

    let mut tx_list_column = Column::new().padding(10).spacing(5);
    tx_list_column = tx_list_column.push(IcedText::new("Transactions").size(18));
    

    let mut sorted_transactions = app.transactions.clone();
    match app.sort_type {
        SortType::NewestFirst => sorted_transactions.sort_by(|a, b| b.date.cmp(&a.date)),
        SortType::OldestFirst => sorted_transactions.sort_by(|a, b| a.date.cmp(&b.date)),
        SortType::OnlyIncome => sorted_transactions.retain(|t| t.tran_type.eq_ignore_ascii_case("income")),
        SortType::OnlyExpense => sorted_transactions.retain(|t| t.tran_type.eq_ignore_ascii_case("expense")),
    }

    for tx in &sorted_transactions {
        let color = if tx.tran_type.eq_ignore_ascii_case("expense") {
            Color::from_rgb(1.0, 0.0, 0.0)
        } else {
            Color::from_rgb(0.0, 0.6, 0.0)
        };
        let formatted_date = tx.date.format("%Y-%m-%d %H:%M:%S").to_string();
        let line = format!("{} {} – {:+.2} [{}]", tx.tran_type, tx.tran_source, tx.tran_amount, formatted_date);
        tx_list_column = tx_list_column.push(IcedText::new(line).style(iced::theme::Text::Color(color)));
    }

    // let tx_list = Scrollable::new(tx_list_column).width(Length::FillPortion(2));
    let tx_list = Container::new(Scrollable::new(tx_list_column))
        .style(iced::theme::Container::Custom(Box::new(TransactionListBackground)))
        .width(Length::FillPortion(2));

    let balance: f64 = app.transactions.iter().map(|tx| {
        if tx.tran_type == "Expense" { -tx.tran_amount } else { tx.tran_amount }
    }).sum();



let top_bar = Container::new(
    Row::new()
        .padding(16)
        .align_items(Alignment::Center)
        .spacing(20)
        .push(IcedText::new(app.user_name.as_deref().unwrap_or(""))
            .size(20)
            .style(Color::WHITE))
        .push(Space::with_width(Length::Fill))
        .push(IcedText::new(format!("Balance: {:+.2}", balance))
            .size(20)
            .style(Color::WHITE))
        .push(Space::with_width(Length::Fill))
        .push(
            Button::new(IcedText::new("Logout"))
                .on_press(Message::SwitchToLogin)
        )
)
    .width(Length::Fill)
    .padding(10)
    .style(iced::theme::Container::Custom(Box::new(BlackBackground)));


    let buttons = Column::new()
        .spacing(10)
        .push(Button::new(IcedText::new("Add Expense")).on_press(Message::ChooseAddExpense))
        .push(Button::new(IcedText::new("Add Income")).on_press(Message::ChooseAddIncome));
        Container::new(
            Column::new()
                .push(top_bar)
                .push(top_controls)
                .push(
                    Row::new()
                        .spacing(40)
                        .push(pie)
                        .push(tx_list)
                        .push(buttons)
                ))
                .style(iced::theme::Container::Custom(Box::new(BodyBackground)))
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
}

fn render_add_expense(app: &CombinedApp) -> Element<Message> {
    Column::new()
        .padding(20)
        .spacing(10)
        .push(
            TextInput::new("Store", &app.store_name)
                .on_input(Message::ChangeStoreName)
        )
        .push(
    TextInput::new("Date (YYYY-MM-DD)", &app.expense_date.format("%Y-%m-%d").to_string())
        .on_input(Message::ChangeExpenseDateString)
        )
        .push(
            Button::new(IcedText::new("Today")).on_press(Message::SetExpenseDateToToday)
        )

        .push(
            TextInput::new("Amount", &app.expense_sum)
                .on_input(Message::ChangeExpenseSum)
        )
        .push(
            PickList::new(
                &app.categories[..],
                app.selected_category.clone(),
                |selected| Message::CategorySelected(Some(selected)),
            )
        )
        .push(
            Row::new().spacing(10)
                .push(Button::new(IcedText::new("Confirm")).on_press(Message::ConfirmAddExpense))
                .push(Button::new(IcedText::new("Cancel")).on_press(Message::CancelDashboardAction))
        )
        .into()
}

fn render_add_income(app: &CombinedApp) -> Element<Message> {
    Column::new()
        .padding(20)
        .spacing(10)
        .push(
            TextInput::new("Source", &app.income_source)
                .on_input(Message::ChangeIncomeSource)
        )

        .push(
    TextInput::new("Date (YYYY-MM-DD)", &app.expense_date.format("%Y-%m-%d").to_string())
        .on_input(Message::ChangeExpenseDateString)
        )
        .push(
            Button::new(IcedText::new("Today")).on_press(Message::SetExpenseDateToToday)
        )
        .push(
            TextInput::new("Amount", &app.income_sum)
                .on_input(Message::ChangeIncomeSum)
        )
        .push(
            Row::new().spacing(10)
                .push(Button::new(IcedText::new("Confirm")).on_press(Message::ConfirmAddIncome))
                .push(Button::new(IcedText::new("Cancel")).on_press(Message::CancelDashboardAction))
        )
        .into()
}
