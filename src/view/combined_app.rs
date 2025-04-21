use iced::{
    executor, Application, Command, Element, Theme, Color, theme,
    widget::{Button, Column, Row, Space, Text as IcedText, TextInput},
    Alignment, Length,
};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::controller::login_controller::attempt_login;
use crate::controller::registration_controller::attempt_register;
use crate::controller::transaction_controller::{load_transactions, add_expense, add_income};
use crate::model::{AuthData, Transaction};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub enum DashboardViewMode {
    Dashboard,
    AddOption,
    AddExpense,
    AddIncome,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Login,
    Registration,
    Dashboard(DashboardViewMode),
}

#[derive(Debug, Clone)]
pub enum Message {
    LoginUsernameChanged(String),
    LoginPasswordChanged(String),
    LoginPressed,
    LoginResult(Result<i32, String>),
    TransactionsLoaded(Vec<Transaction>),
    RegUsernameChanged(String),
    RegPasswordChanged(String),
    RegConfirmChanged(String),
    RegisterPressed,
    RegisterResult(Result<(), String>),
    SwitchToLogin,
    SwitchToRegistration,
    ShowAddOption,
    ChooseAddExpense,
    ChooseAddIncome,
    CancelDashboardAction,
    ChangeStoreName(String),
    ChangeExpenseDate(String),
    ChangeExpenseSum(String),
    ConfirmAddExpense,
    ChangeIncomeSource(String),
    ChangeIncomeDate(String),
    ChangeIncomeSum(String),
    ConfirmAddIncome,
    SettingsPressed,
}

pub struct CombinedApp {
  pub  current_screen: Screen,
  pub  login_username: String,
  pub  login_password: String,
  pub  login_message: String,
  pub  reg_username: String,
  pub  reg_password: String,
  pub  reg_confirm: String,
  pub  reg_message: String,
  pub  user_name: Option<String>,
  pub  user_id: Option<i32>,
  pub  transactions: Vec<Transaction>,
  pub store_name: String,
    pub  expense_date: String,
    pub  expense_sum: String,
    pub  income_source: String,
    pub  income_date: String,
    pub  income_sum: String,
    pub  dashboard_message: String,
    pub  pool: DbPool,
}

impl CombinedApp {
    fn new_pool() -> DbPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager).expect("Failed to create pool")
    }
}

impl Default for CombinedApp {
    fn default() -> Self {
        dotenv().ok();
        CombinedApp {
            current_screen: Screen::Login,
            login_username: String::new(),
            login_password: String::new(),
            login_message: "Enter your login details".into(),
            reg_username: String::new(),
            reg_password: String::new(),
            reg_confirm: String::new(),
            reg_message: "Enter your registration details".into(),


            user_name: None,
            user_id: None,

            dashboard_message: "Your transaction history".into(),
            transactions: Vec::new(),
            store_name: String::new(),
            expense_date: String::new(),
            expense_sum: String::new(),
            income_source: String::new(),
            income_date: String::new(),
            income_sum: String::new(),

            pool: CombinedApp::new_pool(),
        }
    }
}

impl Application for CombinedApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (CombinedApp::default(), Command::none())
    }

    fn title(&self) -> String {
        match self.current_screen {
            Screen::Login => "Login".into(),
            Screen::Registration => "Register".into(),
            Screen::Dashboard(_) => "Dashboard".into(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoginUsernameChanged(v) => self.login_username = v,
            Message::LoginPasswordChanged(v) => self.login_password = v,
            Message::LoginPressed => {
                let user = self.login_username.clone();
                let pass = self.login_password.clone();
                let pool = self.pool.clone();
                return Command::perform(
                    async move { attempt_login(&pool, &AuthData { username: user, password: pass }) },
                    Message::LoginResult,
                );
            }
            Message::LoginResult(Ok(id)) => {
                self.user_name = Some(self.login_username.clone());
                self.user_id = Some(id);
                let pool = self.pool.clone();
                return Command::perform(
                    async move { load_transactions(id).unwrap_or_default() },
                    Message::TransactionsLoaded,
                );
            }
            Message::LoginResult(Err(e)) => self.login_message = e,

            Message::TransactionsLoaded(txs) => {
                self.transactions = txs;
                self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
            }

            Message::RegUsernameChanged(v) => self.reg_username = v,
            Message::RegPasswordChanged(v) => self.reg_password = v,
            Message::RegConfirmChanged(v) => self.reg_confirm = v,
            Message::RegisterPressed => {
                if self.reg_password != self.reg_confirm {
                    self.reg_message = "Passwords do not match".into();
                } else {
                    let user = self.reg_username.clone();
                    let pass = self.reg_password.clone();
                    let pool = self.pool.clone();
                    return Command::perform(
                        async move { attempt_register(&pool, &AuthData { username: user, password: pass }) },
                        Message::RegisterResult,
                    );
                }
            }
            Message::RegisterResult(Ok(())) => self.current_screen = Screen::Login,
            Message::RegisterResult(Err(e)) => self.reg_message = e,

            Message::SwitchToLogin => self.current_screen = Screen::Login,
            Message::SwitchToRegistration => self.current_screen = Screen::Registration,

            Message::ShowAddOption => self.current_screen = Screen::Dashboard(DashboardViewMode::AddOption),
            Message::ChooseAddExpense => self.current_screen = Screen::Dashboard(DashboardViewMode::AddExpense),
            Message::ChooseAddIncome => self.current_screen = Screen::Dashboard(DashboardViewMode::AddIncome),
            Message::CancelDashboardAction => self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard),

            Message::ChangeStoreName(v) => self.store_name = v,
            Message::ChangeExpenseDate(v) => self.expense_date = v,
            Message::ChangeExpenseSum(v) => self.expense_sum = v,
            Message::ConfirmAddExpense => {
                if let Some(uid) = self.user_id {
                    let amt = self.expense_sum.parse().unwrap_or(0.0);
                    let _ = add_expense(&self.pool, uid, &self.store_name, &self.expense_date, amt);
                }
                self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
            }
            Message::ChangeIncomeSource(v) => self.income_source = v,
            Message::ChangeIncomeDate(v) => self.income_date = v,
            Message::ChangeIncomeSum(v) => self.income_sum = v,
            Message::ConfirmAddIncome => {
                if let Some(uid) = self.user_id {
                    let amt = self.income_sum.parse().unwrap_or(0.0);
                    let _ = add_income(&self.pool, uid, &self.income_source, &self.income_date, amt);
                }
                self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
            }

            Message::SettingsPressed => {},
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.current_screen {
            Screen::Login => {
                Column::new()
                    .padding(20)
                    .spacing(15)
                    .align_items(Alignment::Center)
                    .push(IcedText::new("Login"))
                    .push(TextInput::new("Username", &self.login_username)
                        .on_input(Message::LoginUsernameChanged))
                    .push(TextInput::new("Password", &self.login_password)
                        .on_input(Message::LoginPasswordChanged))
                    .push(Button::new(IcedText::new("Login")).on_press(Message::LoginPressed))
                    .push(IcedText::new(&self.login_message))
                    .push(Button::new(IcedText::new("Register")).on_press(Message::SwitchToRegistration))
                    .into()
            }

            Screen::Registration => {
                Column::new()
                    .padding(20)
                    .spacing(15)
                    .align_items(Alignment::Center)
                    .push(IcedText::new("Register"))
                    .push(TextInput::new("Username", &self.reg_username)
                        .on_input(Message::RegUsernameChanged))
                    .push(TextInput::new("Password", &self.reg_password)
                        .on_input(Message::RegPasswordChanged))
                    .push(TextInput::new("Confirm", &self.reg_confirm)
                        .on_input(Message::RegConfirmChanged))
                    .push(Button::new(IcedText::new("Register")).on_press(Message::RegisterPressed))
                    .push(IcedText::new(&self.reg_message))
                    .push(Button::new(IcedText::new("Login")).on_press(Message::SwitchToLogin))
                    .into()
            }

            Screen::Dashboard(mode) => {
                match mode {
                    DashboardViewMode::Dashboard => {
                        // Верхняя панель
                        let top_bar = Row::new()
                            .padding(10)
                            .align_items(Alignment::Center)
                            .push(IcedText::new(match &self.user_name {
                                Some(name) => name,
                                None => "",
                            }))
                            .push(Space::with_width(Length::Fill))
                            .push(Button::new(IcedText::new("Settings")).on_press(Message::SettingsPressed));
                        
                        // Список транзакций цветной
                        let mut records = Column::new().padding(20).spacing(10);
                        for tx in &self.transactions {
                            let color = if tx.tran_type == "Expense" {
                                Color::from_rgb(1.0, 0.0, 0.0)
                            } else {
                                Color::from_rgb(0.0, 1.0, 0.0)
                            };
                            records = records.push(
                                Row::new().spacing(15)
                                    .push(IcedText::new(&tx.tran_type).style(theme::Text::Color(color)))
                                    .push(IcedText::new(&tx.tran_source))
                                    .push(IcedText::new(&tx.date))
                                    .push(IcedText::new(format!("{:.2}", tx.tran_amount)))
                            );
                        }

                        // Кнопки добавить
                        let records_area = records;

                        Column::new()
                            .push(top_bar)
                            .push(IcedText::new(&self.dashboard_message))
                            .push(Space::with_height(Length::Fixed(20.0)))
                            .spacing(20)
                            //Сделать отступы для кнопок и текста

                            .push(Button::new(IcedText::new("Add Expense")).on_press(Message::ChooseAddExpense))
                            .push(Button::new(IcedText::new("Add Income")).on_press(Message::ChooseAddIncome))
                            .push(records_area)
                            .into()
                    }
                    DashboardViewMode::AddOption => {
                        Column::new()
                            .padding(20)
                            .spacing(20)
                            .push(Button::new(IcedText::new("Add Expense")).on_press(Message::ChooseAddExpense))
                            .push(Button::new(IcedText::new("Add Income")).on_press(Message::ChooseAddIncome))
                            .push(Button::new(IcedText::new("Cancel")).on_press(Message::CancelDashboardAction))
                            .into()
                    }
                    DashboardViewMode::AddExpense => {
                        let store_input = TextInput::new("Store Name", &self.store_name)
                            .on_input(Message::ChangeStoreName);
                        let date_input = TextInput::new("Purchase Date", &self.expense_date)
                            .on_input(Message::ChangeExpenseDate);
                        let sum_input = TextInput::new("Expense Amount", &self.expense_sum)
                            .on_input(Message::ChangeExpenseSum);
                        let confirm = Button::new(IcedText::new("Confirm")).on_press(Message::ConfirmAddExpense);
                        let cancel = Button::new(IcedText::new("Cancel")).on_press(Message::CancelDashboardAction);
                        Column::new()
                            .padding(20)
                            .spacing(10)
                            .push(store_input)
                            .push(date_input)
                            .push(sum_input)
                            .push(Row::new().spacing(10).push(confirm).push(cancel))
                            .into()
                    }
                    DashboardViewMode::AddIncome => {
                        let source_input = TextInput::new("Income Source", &self.income_source)
                            .on_input(Message::ChangeIncomeSource);
                        let date_input = TextInput::new("Date", &self.income_date)
                            .on_input(Message::ChangeIncomeDate);
                        let sum_input = TextInput::new("Amount", &self.income_sum)
                            .on_input(Message::ChangeIncomeSum);
                        let confirm = Button::new(IcedText::new("Confirm")).on_press(Message::ConfirmAddIncome);
                        let cancel = Button::new(IcedText::new("Cancel")).on_press(Message::CancelDashboardAction);
                        Column::new()
                            .padding(20)
                            .spacing(10)
                            .push(source_input)
                            .push(date_input)
                            .push(sum_input)
                            .push(Row::new().spacing(10).push(confirm).push(cancel))
                            .into()
                    }
                }
            }
        }
    }
}
