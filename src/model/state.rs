use crate::model::{Transaction, DbPool};
use crate::model::db::create_pool;
use chrono::NaiveDateTime;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub enum DashboardViewMode {
    Main,       
    AddExpense,
    AddIncome,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Login,
    Registration,
    Dashboard(DashboardViewMode),
    ResetPassword,
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
    RequestPasswordReset,
    SecretPassChanged(String),
    NewPasswordChanged(String),
    ConfirmNewPasswordChanged(String),
    SubmitPasswordReset,
    PasswordResetResult(Result<(), String>),
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
    CategorySelected(Option<String>),
    ExitPressed,
    CombinedLoaded((Vec<Transaction>, Vec<String>)),
    SortTypeChanged(SortType),

DeleteTransaction(i32), // i32 = tran_id
TransactionDeleted(Result<(), String>),



    // DateSelectedExpense(iced_aw::date_picker::Date),


    // DateSelectedIncome(NaiveDateTime),


    ChangeExpenseDateString(String),
    SetExpenseDateToToday,


}

pub struct CombinedApp {
    pub current_screen: Screen,
    pub login_username: String,
    pub login_password: String,
    pub login_message: String,
    pub reg_username: String,
    pub reg_password: String,
    pub reg_confirm: String,
    pub reg_message: String,
    pub user_name: Option<String>,
    pub user_id: Option<i32>,
    pub transactions: Vec<Transaction>,
    pub store_name: String,
    // pub show_date_picker_expense: bool,
    // pub show_date_picker_income: bool,
    pub expense_date: NaiveDateTime,
    pub expense_sum: String,
    pub income_source: String,
    pub income_date: NaiveDateTime,
    pub income_sum: String,
    pub secret_pass: String,         // ответ на секретный вопрос
    pub new_password: String,          // ввод нового пароля
    pub confirm_new_password: String,
    pub pool: DbPool,
    // pub tx_list_state: iced::widget::scrollable::State,
    pub categories: Vec<String>,
    pub selected_category: Option<String>,
    pub sort_type: SortType,

    pub expense_date_str: String,
    pub income_date_str: String,


}

impl CombinedApp {
    /// Обновляет транзакции и категории после успешного входа
    pub fn apply_login_result(&mut self, transactions: Vec<Transaction>, categories: Vec<String>) {
        self.transactions = transactions;
        self.categories = categories;
        self.current_screen = Screen::Dashboard(DashboardViewMode::Main);
    }

    /// Очищает все поля формы расходов
    pub fn clear_expense_form(&mut self) {
        self.store_name.clear();
        // self.expense_date.clear();
        self.expense_sum.clear();
        self.selected_category = None;
    }

    /// Очищает все поля формы доходов
    pub fn clear_income_form(&mut self) {
        self.income_source.clear();
        // self.income_date.clear();
        self.income_sum.clear();
    }
}

impl Default for CombinedApp {
    fn default() -> Self {
        dotenv().ok();

        CombinedApp {
            current_screen: Screen::Login,
            login_username: String::new(),
            login_password: String::new(),
            login_message: String::new(),
            reg_username: String::new(),
            reg_password: String::new(),
            reg_confirm: String::new(),
            reg_message: String::new(),
            user_name: None,
            secret_pass: String::new(),
            new_password: String::new(),
            confirm_new_password: String::new(),
            user_id: None,
            // tx_list_state: Default::default(),
            transactions: Vec::new(),
            store_name: String::new(),
            // show_date_picker_expense: false,
            // show_date_picker_income: false,

            expense_date: chrono::Local::now().naive_local(),

            income_date: chrono::Local::now().naive_local(),

            expense_sum: String::new(),
            income_source: String::new(),

            income_sum: String::new(),
            pool: create_pool(),
            categories: vec![],
            selected_category: None,
            sort_type: SortType::NewestFirst,

            expense_date_str: "".to_string(),
            income_date_str: String::new(),


        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortType {
    NewestFirst,
    OldestFirst,
    OnlyIncome,
    OnlyExpense,
}

impl SortType {
    pub const ALL: [SortType; 4] = [
        SortType::NewestFirst,
        SortType::OldestFirst,
        SortType::OnlyIncome,
        SortType::OnlyExpense,
    ];
}

impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortType::NewestFirst => write!(f, "New first"),
            SortType::OldestFirst => write!(f, "First old"),
            SortType::OnlyIncome => write!(f, "Income only"),
            SortType::OnlyExpense => write!(f, "Expenses only"),
        }
    }
}
