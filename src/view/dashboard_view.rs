// src/view/dashboard_view.rs

use iced::{
    widget::{Button, Canvas, Column, PickList, Row, Scrollable, Space, Text as IcedText, TextInput},
    Alignment, Color, Element, Length, Point, Rectangle,
    mouse::Cursor, Renderer, Theme,
};
use iced::widget::canvas::{Frame, Path, Program, Geometry, Text as CanvasText};
use iced::widget::canvas::path::Arc as CanvasArc;
use std::collections::HashMap;
use std::f32::consts::PI;
use crate::model::state::SortType;

use crate::model::{CombinedApp, DashboardViewMode, Message};

pub fn render<'a>(
    app: &'a CombinedApp,
    mode: &'a DashboardViewMode,
) -> Element<'a, Message> {
    match mode {
        DashboardViewMode::Main       => render_dashboard_main(&app),
        DashboardViewMode::AddExpense => render_add_expense(&app),
        DashboardViewMode::AddIncome  => render_add_income(&app),
    }
}

fn render_dashboard_main(app: &CombinedApp) -> Element<Message> {
    // 1) Считаем сумму расходов по категориям
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

    // 2) Превращаем в вектор и отбрасываем нулевые категории
    let chart_data: Vec<(String, f32)> = totals
        .into_iter()
        .filter(|(_, sum)| sum > &0.0)
        .collect();

    // 3) Программа для рисования «пирога»
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
            let radius = center.x.min(center.y) * 0.8;
            let total: f32 = self.data.iter().map(|(_, v)| *v).sum();

            let mut start_angle = 0.0_f32;
            for (i, (label, value)) in self.data.iter().enumerate() {
                // Доля сегмента (pct) и угол в радианах (sweep)
                let pct = if total > 0.0 { value / total } else { 0.0 };
                let sweep = pct * 2.0 * PI;
                let end_angle = start_angle + sweep;

                // Первая точка дуги на окружности (начало сегмента)
                let sx = center.x + radius * start_angle.cos();
                let sy = center.y + radius * start_angle.sin();

                let path = Path::new(|p| {
                    p.move_to(center);
                    p.line_to(Point::new(sx, sy));
                    p.arc(CanvasArc {
                        center,
                        radius,
                        start_angle,
                        end_angle,
                    });
                    p.close(); // замыкаем обратно к центру
                });

                // Цветовая палитра (5 цветов по кругу)
                let color = match i % 5 {
                    0 => Color::from_rgb(0.9, 0.3, 0.3),
                    1 => Color::from_rgb(0.3, 0.9, 0.3),
                    2 => Color::from_rgb(0.3, 0.3, 0.9),
                    3 => Color::from_rgb(0.9, 0.9, 0.3),
                    _ => Color::from_rgb(0.7, 0.3, 0.9),
                };
                frame.fill(&path, color);

                // Подпись в центре сектора (метка и процент)
                let mid_angle = start_angle + sweep / 2.0;
                let tx = center.x + radius * 0.6 * mid_angle.cos();
                let ty = center.y + radius * 0.6 * mid_angle.sin();
                frame.fill_text(CanvasText {
                    content: format!("{} ({:.0}%)", label, pct * 100.0),
                    position: Point::new(tx, ty),
                    color: Color::BLACK,
                    size: 14.0,
                    ..CanvasText::default()
                });

                start_angle = end_angle;
            }

            vec![frame.into_geometry()]
        }
    }

    // 4) Создаём Canvas с диаграммой
    let pie = Canvas::new(PieChart { data: chart_data })
        .width(Length::Fixed(250.0))
        .height(Length::Fixed(250.0));

    // 5) Прокручиваемый список транзакций
    // Формируем колонку со списком операций
    let sort_picker = PickList::new(
    &SortType::ALL[..],
    Some(app.sort_type.clone()),
    Message::SortTypeChanged,
);

let top_controls = Row::new()
    .padding(10)
    .spacing(20)
    .push(IcedText::new("Сортировка:"))
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

    let category = tx.tran_comment.clone().unwrap_or_else(|| "-".to_string());

    let line = format!("{} {} – {:+.2} [{}] | {}", tx.tran_type, tx.tran_source, tx.tran_amount, tx.date, category);
    tx_list_column = tx_list_column.push(
        IcedText::new(line).style(iced::theme::Text::Color(color))
    );
}

    // Оборачиваем колонку в Scrollable (ширина занимает 2/4 части строки)
    let tx_list = Scrollable::new(tx_list_column).width(Length::FillPortion(2));

    // 6) Верхняя панель: имя пользователя, баланс, кнопка Logout
    let balance: f64 = app.transactions.iter().map(|tx| {
        if tx.tran_type == "Expense" { -tx.tran_amount } else { tx.tran_amount }
    }).sum();
    let top_bar = Row::new()
        .padding(16)
        .align_items(Alignment::Center)
        .spacing(20)
        .push(IcedText::new(app.user_name.as_deref().unwrap_or("")).size(20))
        .push(Space::with_width(Length::Fill))
        .push(IcedText::new(format!("Balance: {:+.2}", balance)).size(20))
        .push(Space::with_width(Length::Fill))
        .push(Button::new(IcedText::new("Logout")).on_press(Message::SwitchToLogin));

    // 7) Правая колонка с кнопками "Add Expense" / "Add Income"
    let buttons = Column::new()
        .spacing(10)
        .push(Button::new(IcedText::new("Add Expense")).on_press(Message::ChooseAddExpense))
        .push(Button::new(IcedText::new("Add Income")).on_press(Message::ChooseAddIncome));

    // 8) Компонуем основной вид: диаграмма, список транзакций, кнопки действий
    Column::new()
        .push(top_bar)
        .push(top_controls)  // ← Добавь это

        .push(
            Row::new()
                .spacing(40)
                .push(pie)
                .push(tx_list)
                .push(buttons)
        )
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
            TextInput::new("Date (YYYY-MM-DD)", &app.expense_date)
                .on_input(Message::ChangeExpenseDate)
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
            TextInput::new("Date (YYYY-MM-DD)", &app.income_date)
                .on_input(Message::ChangeIncomeDate)
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
