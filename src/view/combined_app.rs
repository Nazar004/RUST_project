use iced::{
    executor, Application, Command, Element, Settings, Theme,
    widget::{Button, Column, Row, Space, Text, TextInput},
    Alignment, Length,
};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::controller::login_controller::attempt_login;
use crate::controller::registration_controller::attempt_register;use crate::model::AuthData;

/// Alias для пула соединений
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
    // Экран входа
    LoginUsernameChanged(String),
    LoginPasswordChanged(String),
    LoginPressed,
    LoginResult(Result<(), String>),
    SwitchToRegistration,
    // Экран регистрации
    RegUsernameChanged(String),
    RegPasswordChanged(String),
    RegConfirmChanged(String),
    RegisterPressed,
    RegisterResult(Result<(), String>),
    SwitchToLogin,
    // Дашборд – для выбора добавления записи
    ShowAddOption,
    ChooseAddExpense,
    ChooseAddIncome,
    // Форма расходов
    ChangeStoreName(String),
    ChangeExpenseDate(String),
    ChangeExpenseSum(String),
    ConfirmAddExpense,
    // Форма прибыли
    ChangeSource(String),
    ChangeIncomeDate(String),
    ChangeIncomeSum(String),
    ConfirmAddIncome,
    // Возврат к базовому режиму дашборда
    CancelDashboardAction,
    // Настройки (placeholder)
    SettingsPressed,
}

pub struct CombinedApp {
    pub current_screen: Screen,
    // Поля для экрана входа
    pub login_username: String,
    pub login_password: String,
    pub login_message: String,
    // Поля для экрана регистрации
    pub reg_username: String,
    pub reg_password: String,
    pub reg_confirm: String,
    pub reg_message: String,
    // Поля для дашборда
    pub user_name: Option<String>,
    // Поля для формы добавления расходов
    pub store_name: String,
    pub expense_date: String,
    pub expense_sum: String,
    // Поля для формы добавления прибыли
    pub income_source: String,
    pub income_date: String,
    pub income_sum: String,
    pub dashboard_message: String,
    pub pool: DbPool,
}

impl CombinedApp {
    fn new_pool() -> DbPool {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set.");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .build(manager)
            .expect("Failed to create connection pool")
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
            store_name: String::new(),
            expense_date: String::new(),
            expense_sum: String::new(),
            income_source: String::new(),
            income_date: String::new(),
            income_sum: String::new(),
            dashboard_message: "Dashboard records will appear here".into(),
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
            // --- Обработка экрана входа ---
            Message::LoginUsernameChanged(val) => self.login_username = val,
            Message::LoginPasswordChanged(val) => self.login_password = val,
            Message::LoginPressed => {
                let username = self.login_username.clone();
                let password = self.login_password.clone();
                let pool = self.pool.clone();
                return Command::perform(
                    async move { attempt_login(&pool, &AuthData { username, password }) },
                    Message::LoginResult,
                );
            }
            Message::LoginResult(result) => {
                match result {
                    Ok(_) => {
                        self.login_message = "Successful login".into();
                        self.user_name = Some(self.login_username.clone());
                        // После успешного входа переходим на Dashboard в базовом режиме
                        self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
                    }
                    Err(e) => {
                        self.login_message = format!("Error: {}", e);
                    }
                }
            }
            Message::SwitchToRegistration => {
                self.current_screen = Screen::Registration;
                self.login_message.clear();
            }
            // --- Обработка экрана регистрации ---
            Message::RegUsernameChanged(val) => self.reg_username = val,
            Message::RegPasswordChanged(val) => self.reg_password = val,
            Message::RegConfirmChanged(val) => self.reg_confirm = val,
            Message::RegisterPressed => {
                if self.reg_password != self.reg_confirm {
                    self.reg_message = "Passwords do not match".into();
                } else {
                    let username = self.reg_username.clone();
                    let password = self.reg_password.clone();
                    let pool = self.pool.clone();
                    return Command::perform(
                        async move { attempt_register(&pool, &AuthData { username, password }) },
                        Message::RegisterResult,
                    );
                }
            }
            Message::RegisterResult(result) => {
                match result {
                    Ok(_) => {
                        self.reg_message = "Registration successful!".into();
                        self.current_screen = Screen::Login;
                        self.login_username = self.reg_username.clone();
                        self.reg_username.clear();
                        self.reg_password.clear();
                        self.reg_confirm.clear();
                    }
                    Err(e) => {
                        self.reg_message = format!("Error: {}", e);
                    }
                }
            }
            Message::SwitchToLogin => {
                self.current_screen = Screen::Login;
                self.reg_message.clear();
            }
            // --- Обработка Dashboard ---
            Message::ShowAddOption => {
                self.current_screen = Screen::Dashboard(DashboardViewMode::AddOption);
            }
            Message::ChooseAddExpense => {
                self.current_screen = Screen::Dashboard(DashboardViewMode::AddExpense);
            }
            Message::ChooseAddIncome => {
                self.current_screen = Screen::Dashboard(DashboardViewMode::AddIncome);
            }
            Message::ChangeStoreName(val) => self.store_name = val,
            Message::ChangeExpenseDate(val) => self.expense_date = val,
            Message::ChangeExpenseSum(val) => self.expense_sum = val,
            Message::ConfirmAddExpense => {
                self.dashboard_message = format!(
                    "Expense added:\nStore: {}, Date: {}, Amount: {}",
                    self.store_name, self.expense_date, self.expense_sum
                );
                self.store_name.clear();
                self.expense_date.clear();
                self.expense_sum.clear();
                self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
            }
            Message::ChangeSource(val) => self.income_source = val,
            Message::ChangeIncomeDate(val) => self.income_date = val,
            Message::ChangeIncomeSum(val) => self.income_sum = val,
            Message::ConfirmAddIncome => {
                self.dashboard_message = format!(
                    "Income added:\nSource: {}, Date: {}, Amount: {}",
                    self.income_source, self.income_date, self.income_sum
                );
                self.income_source.clear();
                self.income_date.clear();
                self.income_sum.clear();
                self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
            }
            Message::CancelDashboardAction => {
                self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
                self.dashboard_message.clear();
            }
            Message::SettingsPressed => {
                // Placeholder для настроек
            }
        }
        Command::none()
    }
    
    fn view(&self) -> Element<Message> {
        match &self.current_screen {
            Screen::Login => {
                let username_input = TextInput::new("Username", &self.login_username)
                    .on_input(Message::LoginUsernameChanged);
                let password_input = TextInput::new("Password", &self.login_password)
                    .on_input(Message::LoginPasswordChanged);
                let login_button = Button::new(Text::new("Login"))
                    .on_press(Message::LoginPressed);
                let registration_button = Button::new(Text::new("Register"))
                    .on_press(Message::SwitchToRegistration);
                Column::new()
                    .padding(20)
                    .spacing(15)
                    .align_items(Alignment::Center)
                    .push(Text::new("Login"))
                    .push(username_input)
                    .push(password_input)
                    .push(login_button)
                    .push(registration_button)
                    .push(Text::new(&self.login_message))
                    .into()
            }
            Screen::Registration => {
                let username_input = TextInput::new("Username", &self.reg_username)
                    .on_input(Message::RegUsernameChanged);
                let password_input = TextInput::new("Password", &self.reg_password)
                    .on_input(Message::RegPasswordChanged);
                let confirm_input = TextInput::new("Confirm Password", &self.reg_confirm)
                    .on_input(Message::RegConfirmChanged);
                let register_button = Button::new(Text::new("Register"))
                    .on_press(Message::RegisterPressed);
                let login_button = Button::new(Text::new("Login"))
                    .on_press(Message::SwitchToLogin);
                Column::new()
                    .padding(20)
                    .spacing(15)
                    .align_items(Alignment::Center)
                    .push(Text::new("Register"))
                    .push(username_input)
                    .push(password_input)
                    .push(confirm_input)
                    .push(register_button)
                    .push(login_button)
                    .push(Text::new(&self.reg_message))
                    .into()
            }
            Screen::Dashboard(mode) => {
                match mode {
                    DashboardViewMode::Dashboard => {
                        // Верхняя панель: имя пользователя слева, настройки справа
                        let top_bar = Row::new()
                            .padding(10)
                            .align_items(Alignment::Center)
                            .push(Text::new(match &self.user_name {
                                Some(name) => name,
                                None => "",
                            }))
                            .push(Space::with_width(Length::Fill))
                            .push(Button::new(Text::new("Settings"))
                                .on_press(Message::SettingsPressed));
                        
                        // Основной контент: список записей (placeholder) и кнопка "Add Record"
                        let records_area = Column::new()
                            .padding(20)
                            .spacing(10)
                            .align_items(Alignment::Center)
                            .push(Text::new("Record list (placeholder)"))
                            .push(Button::new(Text::new("Add Record"))
                                .on_press(Message::ShowAddOption));
                        
                        Column::new()
                            .push(top_bar)
                            .push(records_area)
                            .push(Text::new(&self.dashboard_message))
                            .into()
                    }
                    DashboardViewMode::AddOption => {
                        // Выбор между добавлением расходов и прибылей
                        let expense_button = Button::new(Text::new("Add Expense"))
                            .on_press(Message::ChooseAddExpense);
                        let income_button = Button::new(Text::new("Add Income"))
                            .on_press(Message::ChooseAddIncome);
                        Column::new()
                            .padding(20)
                            .spacing(20)
                            .push(expense_button)
                            .push(income_button)
                            .push(Button::new(Text::new("Cancel"))
                                .on_press(Message::CancelDashboardAction))
                            .into()
                    }
                    DashboardViewMode::AddExpense => {
                        // Форма добавления расходов
                        let store_input = TextInput::new("Store Name", &self.store_name)
                            .on_input(Message::ChangeStoreName);
                        let date_input = TextInput::new("Purchase Date", &self.expense_date)
                            .on_input(Message::ChangeExpenseDate);
                        let sum_input = TextInput::new("Expense Amount", &self.expense_sum)
                            .on_input(Message::ChangeExpenseSum);
                        let confirm_button = Button::new(Text::new("Confirm"))
                            .on_press(Message::ConfirmAddExpense);
                        let cancel_button = Button::new(Text::new("Cancel"))
                            .on_press(Message::CancelDashboardAction);
                        Column::new()
                            .padding(20)
                            .spacing(10)
                            .push(store_input)
                            .push(date_input)
                            .push(sum_input)
                            .push(Row::new().spacing(10).push(confirm_button).push(cancel_button))
                            .into()
                    }
                    DashboardViewMode::AddIncome => {
                        // Форма добавления прибыли
                        let source_input = TextInput::new("Income Source", &self.income_source)
                            .on_input(Message::ChangeSource);
                        let date_input = TextInput::new("Date", &self.income_date)
                            .on_input(Message::ChangeIncomeDate);
                        let sum_input = TextInput::new("Amount", &self.income_sum)
                            .on_input(Message::ChangeIncomeSum);
                        let confirm_button = Button::new(Text::new("Confirm"))
                            .on_press(Message::ConfirmAddIncome);
                        let cancel_button = Button::new(Text::new("Cancel"))
                            .on_press(Message::CancelDashboardAction);
                        Column::new()
                            .padding(20)
                            .spacing(10)
                            .push(source_input)
                            .push(date_input)
                            .push(sum_input)
                            .push(Row::new().spacing(10).push(confirm_button).push(cancel_button))
                            .into()
                    }
                }
            }
        }
    }
}



// use iced::{
//     executor, Application, Command, Element, Settings, Theme,
//     widget::{Button, Column, Row, Space, Text, TextInput},
//     Alignment, Length,
// };
// use diesel::r2d2::{Pool, ConnectionManager};
// use diesel::PgConnection;
// use dotenv::dotenv;
// use std::env;

// use crate::controller::login_controller::attempt_login;
// use crate::controller::registration_controller::attempt_register;
// use crate::model::AuthData;

// // Импортируем функцию отрисовки дашборда из отдельного файла
// use crate::view::dashboard::view_dashboard;

// pub type DbPool = Pool<ConnectionManager<PgConnection>>;

// #[derive(Debug, Clone)]
// pub enum Message {
//     // Экран входа
//     LoginUsernameChanged(String),
//     LoginPasswordChanged(String),
//     LoginPressed,
//     LoginResult(Result<(), String>),
//     SwitchToRegistration,
//     // Экран регистрации
//     RegUsernameChanged(String),
//     RegPasswordChanged(String),
//     RegConfirmChanged(String),
//     RegisterPressed,
//     RegisterResult(Result<(), String>),
//     SwitchToLogin,
//     // Дашборд
//     ShowAddOption,
//     ChooseAddExpense,
//     ChooseAddIncome,
//     ChangeStoreName(String),
//     ChangeExpenseDate(String),
//     ChangeExpenseSum(String),
//     ConfirmAddExpense,
//     ChangeSource(String),
//     ChangeIncomeDate(String),
//     ChangeIncomeSum(String),
//     ConfirmAddIncome,
//     CancelDashboardAction,
//     SettingsPressed,
// }

// #[derive(Debug, Clone)]
// pub enum Screen {
//     Login,
//     Registration,
//     Dashboard(DashboardViewMode),
// }

// #[derive(Debug, Clone)]
// pub enum DashboardViewMode {
//     Dashboard,
//     AddOption,
//     AddExpense,
//     AddIncome,
// }

// pub struct CombinedApp {
//     pub current_screen: Screen,
//     // Поля для экрана входа
//     pub login_username: String,
//     pub login_password: String,
//     pub login_message: String,
//     // Поля для экрана регистрации
//     pub reg_username: String,
//     pub reg_password: String,
//     pub reg_confirm: String,
//     pub reg_message: String,
//     // Поля для дашборда
//     pub user_name: Option<String>,
//     pub store_name: String,
//     pub expense_date: String,
//     pub expense_sum: String,
//     pub income_source: String,
//     pub income_date: String,
//     pub income_sum: String,
//     pub dashboard_message: String,
//     pub pool: DbPool,
// }

// impl CombinedApp {
//     fn new_pool() -> DbPool {
//         let database_url = env::var("DATABASE_URL")
//             .expect("DATABASE_URL is not set.");
//         let manager = ConnectionManager::<PgConnection>::new(database_url);
//         Pool::builder()
//             .build(manager)
//             .expect("Failed to create connection pool")
//     }
// }

// impl Default for CombinedApp {
//     fn default() -> Self {
//         dotenv().ok();
//         CombinedApp {
//             current_screen: Screen::Login,
//             login_username: String::new(),
//             login_password: String::new(),
//             login_message: "Enter your login details".into(),
//             reg_username: String::new(),
//             reg_password: String::new(),
//             reg_confirm: String::new(),
//             reg_message: "Enter your registration details".into(),
//             user_name: None,
//             store_name: String::new(),
//             expense_date: String::new(),
//             expense_sum: String::new(),
//             income_source: String::new(),
//             income_date: String::new(),
//             income_sum: String::new(),
//             dashboard_message: "Dashboard records will appear here".into(),
//             pool: CombinedApp::new_pool(),
//         }
//     }
// }

// impl Application for CombinedApp {
//     type Executor = executor::Default;
//     type Message = Message;
//     type Flags = ();
//     type Theme = Theme;
    
//     fn new(_flags: ()) -> (Self, Command<Message>) {
//         (CombinedApp::default(), Command::none())
//     }
    
//     fn title(&self) -> String {
//         match self.current_screen {
//             Screen::Login => "Login".into(),
//             Screen::Registration => "Register".into(),
//             Screen::Dashboard(_) => "Dashboard".into(),
//         }
//     }
    
//     fn update(&mut self, message: Message) -> Command<Message> {
//         match message {
//             // --- Экран входа ---
//             Message::LoginUsernameChanged(val) => self.login_username = val,
//             Message::LoginPasswordChanged(val) => self.login_password = val,
//             Message::LoginPressed => {
//                 let username = self.login_username.clone();
//                 let password = self.login_password.clone();
//                 let pool = self.pool.clone();
//                 return Command::perform(
//                     async move { attempt_login(&pool, &AuthData { username, password }) },
//                     Message::LoginResult,
//                 );
//             }
//             Message::LoginResult(result) => {
//                 match result {
//                     Ok(_) => {
//                         self.login_message = "Successful login".into();
//                         self.user_name = Some(self.login_username.clone());
//                         self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
//                     }
//                     Err(e) => {
//                         self.login_message = format!("Error: {}", e);
//                     }
//                 }
//             }
//             Message::SwitchToRegistration => {
//                 self.current_screen = Screen::Registration;
//                 self.login_message.clear();
//             }
//             // --- Экран регистрации ---
//             Message::RegUsernameChanged(val) => self.reg_username = val,
//             Message::RegPasswordChanged(val) => self.reg_password = val,
//             Message::RegConfirmChanged(val) => self.reg_confirm = val,
//             Message::RegisterPressed => {
//                 if self.reg_password != self.reg_confirm {
//                     self.reg_message = "Passwords do not match".into();
//                 } else {
//                     let username = self.reg_username.clone();
//                     let password = self.reg_password.clone();
//                     let pool = self.pool.clone();
//                     return Command::perform(
//                         async move { attempt_register(&pool, &AuthData { username, password }) },
//                         Message::RegisterResult,
//                     );
//                 }
//             }
//             Message::RegisterResult(result) => {
//                 match result {
//                     Ok(_) => {
//                         self.reg_message = "Registration successful!".into();
//                         self.current_screen = Screen::Login;
//                         self.login_username = self.reg_username.clone();
//                         self.reg_username.clear();
//                         self.reg_password.clear();
//                         self.reg_confirm.clear();
//                     }
//                     Err(e) => {
//                         self.reg_message = format!("Error: {}", e);
//                     }
//                 }
//             }
//             Message::SwitchToLogin => {
//                 self.current_screen = Screen::Login;
//                 self.reg_message.clear();
//             }
//             // --- Обработка Dashboard ---
//             Message::ShowAddOption => {
//                 self.current_screen = Screen::Dashboard(DashboardViewMode::AddOption);
//             }
//             Message::ChooseAddExpense => {
//                 self.current_screen = Screen::Dashboard(DashboardViewMode::AddExpense);
//             }
//             Message::ChooseAddIncome => {
//                 self.current_screen = Screen::Dashboard(DashboardViewMode::AddIncome);
//             }
//             Message::ChangeStoreName(val) => self.store_name = val,
//             Message::ChangeExpenseDate(val) => self.expense_date = val,
//             Message::ChangeExpenseSum(val) => self.expense_sum = val,
//             Message::ConfirmAddExpense => {
//                 self.dashboard_message = format!(
//                     "Expense added:\nStore: {}, Date: {}, Amount: {}",
//                     self.store_name, self.expense_date, self.expense_sum
//                 );
//                 self.store_name.clear();
//                 self.expense_date.clear();
//                 self.expense_sum.clear();
//                 self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
//             }
//             Message::ChangeSource(val) => self.income_source = val,
//             Message::ChangeIncomeDate(val) => self.income_date = val,
//             Message::ChangeIncomeSum(val) => self.income_sum = val,
//             Message::ConfirmAddIncome => {
//                 self.dashboard_message = format!(
//                     "Income added:\nSource: {}, Date: {}, Amount: {}",
//                     self.income_source, self.income_date, self.income_sum
//                 );
//                 self.income_source.clear();
//                 self.income_date.clear();
//                 self.income_sum.clear();
//                 self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
//             }
//             Message::CancelDashboardAction => {
//                 self.current_screen = Screen::Dashboard(DashboardViewMode::Dashboard);
//                 self.dashboard_message.clear();
//             }
//             Message::SettingsPressed => {
//                 // Настройки: функционал можно добавить позже
//             }
//         }
//         Command::none()
//     }
    
//     fn view(&self) -> Element<Message> {
//         match &self.current_screen {
//             Screen::Login => {
//                 // Экран входа
//                 let username_input = TextInput::new("Username", &self.login_username)
//                     .on_input(Message::LoginUsernameChanged);
//                 let password_input = TextInput::new("Password", &self.login_password)
//                     .on_input(Message::LoginPasswordChanged);
//                 let login_button = Button::new(Text::new("Login"))
//                     .on_press(Message::LoginPressed);
//                 let registration_button = Button::new(Text::new("Register"))
//                     .on_press(Message::SwitchToRegistration);
//                 Column::new()
//                     .padding(20)
//                     .spacing(15)
//                     .align_items(Alignment::Center)
//                     .push(Text::new("Login"))
//                     .push(username_input)
//                     .push(password_input)
//                     .push(login_button)
//                     .push(registration_button)
//                     .push(Text::new(&self.login_message))
//                     .into()
//             }
//             Screen::Registration => {
//                 // Экран регистрации
//                 let username_input = TextInput::new("Username", &self.reg_username)
//                     .on_input(Message::RegUsernameChanged);
//                 let password_input = TextInput::new("Password", &self.reg_password)
//                     .on_input(Message::RegPasswordChanged);
//                 let confirm_input = TextInput::new("Confirm Password", &self.reg_confirm)
//                     .on_input(Message::RegConfirmChanged);
//                 let register_button = Button::new(Text::new("Register"))
//                     .on_press(Message::RegisterPressed);
//                 let login_button = Button::new(Text::new("Login"))
//                     .on_press(Message::SwitchToLogin);
//                 Column::new()
//                     .padding(20)
//                     .spacing(15)
//                     .align_items(Alignment::Center)
//                     .push(Text::new("Register"))
//                     .push(username_input)
//                     .push(password_input)
//                     .push(confirm_input)
//                     .push(register_button)
//                     .push(login_button)
//                     .push(Text::new(&self.reg_message))
//                     .into()
//             }
//             Screen::Dashboard(_) => {
//                 // Делегируем отрисовку главного экрана (дашборда) в отдельный модуль
//                 view_dashboard(self)
//             }
//         }
//     }
// }
