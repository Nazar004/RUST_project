use iced::{
    executor, Application, Command, Element, Theme
};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::model::AuthData;
use crate::controller::login_controller::attempt_login;
use crate::controller::registration_controller::attempt_register;

// Объявляем тип пула соединений
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub enum Message {
    // Сообщения для экрана входа
    LoginUsernameChanged(String),
    LoginPasswordChanged(String),
    LoginPressed,
    LoginResult(Result<(), String>),

    // Сообщения для экрана регистрации
    RegUsernameChanged(String),
    RegPasswordChanged(String),
    RegConfirmChanged(String),
    RegisterPressed,
    RegisterResult(Result<(), String>),

    // Переключение между экранами
    SwitchToRegistration,
    SwitchToLogin,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Login,
    Registration,
}

pub struct CombinedApp {
    pub current_screen: Screen,
    // Поля для входа
    pub login_username: String,
    pub login_password: String,
    pub login_message: String,
    // Поля для регистрации
    pub reg_username: String,
    pub reg_password: String,
    pub reg_confirm: String,
    pub reg_message: String,
    pub pool: DbPool,
}

impl Application for CombinedApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set.");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DbPool = Pool::builder()
            .build(manager)
            .expect("Failed to create connection pool");

        (
            CombinedApp {
                current_screen: Screen::Login,
                login_username: String::new(),
                login_password: String::new(),
                login_message: "Введите данные для входа".into(),
                reg_username: String::new(),
                reg_password: String::new(),
                reg_confirm: String::new(),
                reg_message: "Введите данные для регистрации".into(),
                pool,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        match self.current_screen {
            Screen::Login => "Вход в систему".into(),
            Screen::Registration => "Регистрация".into(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // Обработка сообщений для входа
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
                self.login_message = match result {
                    Ok(_) => "Успешный вход!".into(),
                    Err(err) => format!("Ошибка: {}", err),
                };
            }
            // Обработка сообщений для регистрации
            Message::RegUsernameChanged(val) => self.reg_username = val,
            Message::RegPasswordChanged(val) => self.reg_password = val,
            Message::RegConfirmChanged(val) => self.reg_confirm = val,
            Message::RegisterPressed => {
                if self.reg_password != self.reg_confirm {
                    self.reg_message = "Пароли не совпадают".into();
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
                self.reg_message = match result {
                    Ok(_) => "Регистрация успешна!".into(),
                    Err(err) => format!("Ошибка: {}", err),
                };
            }
            // Переключение экранов
            Message::SwitchToRegistration => self.current_screen = Screen::Registration,
            Message::SwitchToLogin => self.current_screen = Screen::Login,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Здесь мы делегируем отрисовку соответствующему модулю:
        match self.current_screen {
            Screen::Login => crate::view::login_view::view(self),
            Screen::Registration => crate::view::registration_view::view(self),
        }
    }
}
