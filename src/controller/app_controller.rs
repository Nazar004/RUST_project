use iced::Command;
use crate::controller::login_controller::attempt_password_reset;
use crate::controller::transaction_controller::delete_transaction;
use crate::model::{CombinedApp, Message, Screen, DashboardViewMode, AuthData};
use crate::controller::{
    login_controller::{attempt_login, handle_successful_login},
    registration_controller::attempt_register,
    transaction_controller::{add_expense, add_income},
};
use chrono:: NaiveDate;

pub fn update(app: &mut CombinedApp, message: Message) -> Command<Message> {
    use Message::*;

    match message {
        LoginUsernameChanged(v) => app.login_username = v,
        LoginPasswordChanged(v) => app.login_password = v,
        LoginPressed => {
                let user = app.login_username.clone();
                let pass = app.login_password.clone();
                let secretpass = String::new();
                let pool = app.pool.clone();
                return Command::perform(
                    async move {
                        attempt_login(&pool, &AuthData { username: user, password: pass, secret_pass: secretpass })
                    },
                    LoginResult,
                );
            }
        RequestPasswordReset => {
                app.current_screen = Screen::ResetPassword;
            }
        SecretPassChanged(new_secret) => {
                app.secret_pass = new_secret;
            }
        NewPasswordChanged(new_pass) => {
                app.new_password = new_pass;
            }
        ConfirmNewPasswordChanged(confirm_pass) => {
                app.confirm_new_password = confirm_pass;
            }
        PasswordResetResult(Ok(())) => {
                app.reg_message = "Password has been reset. Please login.".into();
                app.current_screen = Screen::Login;
            }
        PasswordResetResult(Err(e)) => {
                app.reg_message = e;
            }
        

        Message::DeleteTransaction(tx_id) => {
            let pool = app.pool.clone();
            return Command::perform(
        async move {
            delete_transaction(&pool, tx_id).map_err(|e| e.to_string())
                },
        Message::TransactionDeleted,
            );
        }

        Message::TransactionDeleted(Ok(())) => {
            if let Some(uid) = app.user_id {
                let pool = app.pool.clone();
                return Command::perform(
                    async move { handle_successful_login(&pool, uid).await },
                    Message::CombinedLoaded,
                );
            }
        }

        Message::TransactionDeleted(Err(e)) => {
            println!("Error: {}", e);
        }
        LoginResult(Ok(id)) => {
                app.user_name = Some(app.login_username.clone());
                app.user_id = Some(id);
                let pool = app.pool.clone();
                return Command::perform(
                    async move { handle_successful_login(&pool, id).await },
                    CombinedLoaded,
                );
            }
        LoginResult(Err(e)) => app.login_message = e,
        CombinedLoaded((txs, cats)) => app.apply_login_result(txs, cats),
        RegUsernameChanged(v) => app.reg_username = v,
        RegPasswordChanged(v) => app.reg_password = v,
        RegConfirmChanged(v) => app.reg_confirm = v,
        RegisterPressed => {
            if app.reg_password != app.reg_confirm {
                app.reg_message = "Passwords do not match".into();
            } else if app.reg_password.len() < 6 {
                app.reg_message = "Password must be at least 6 characters".into();
            } else if !app.reg_password.chars().any(|c| c.is_uppercase()) {
                app.reg_message = "Password must contain at least one uppercase letter".into();
            } else if !app.reg_password.chars().any(|c| c.is_ascii_digit()) {
                app.reg_message = "Password must contain at least one number".into();
            } else {
                let user = app.reg_username.clone();
                let pass = app.reg_password.clone();
                let secret = app.secret_pass.clone();
                let pool = app.pool.clone();
            
                return Command::perform(
                    async move {
                        attempt_register(&pool, &AuthData {
                            username: user,
                            password: pass,
                            secret_pass: secret,
                        })
                    },
                RegisterResult,
                );
            }
        }
        RegisterResult(Ok(())) => app.current_screen = Screen::Login,
        RegisterResult(Err(e)) => app.reg_message = e,
        SwitchToLogin => {
                app.current_screen = Screen::Login;
                app.user_id = None;
                app.user_name = None;
                app.transactions.clear();
                app.login_password.clear();
            }
        SwitchToRegistration => app.current_screen = Screen::Registration,
        ChooseAddExpense => app.current_screen = Screen::Dashboard(DashboardViewMode::AddExpense),
        ChooseAddIncome => app.current_screen = Screen::Dashboard(DashboardViewMode::AddIncome),
        CancelDashboardAction => app.current_screen = Screen::Dashboard(DashboardViewMode::Main),
        ChangeStoreName(v) => app.store_name = v,

        ChangeExpenseSum(v) => app.expense_sum = v,
        ChangeIncomeSource(v) => app.income_source = v,
        ChangeExpenseDate(v) => {
                if let Ok(parsed) = NaiveDate::parse_from_str(&v, "%Y-%m-%d") {
                    app.income_date = parsed.and_hms_opt(0, 0, 0).unwrap();
                }
            },
        ChangeIncomeSum(v) => app.income_sum = v,
        CategorySelected(cat) => app.selected_category = cat,
        ConfirmAddExpense => {
                if let Some(uid) = app.user_id {
                    let store = app.store_name.clone();
                    let date = chrono::Local::now().naive_local();

                    let amt = app.expense_sum.parse().unwrap_or(0.0);
                    let tag_id = app.categories.iter()
                        .position(|c| Some(c.clone()) == app.selected_category)
                        .map(|i| (i + 1) as i32);
                    let pool = app.pool.clone();

                    app.clear_expense_form();

                    return Command::perform(
                        async move {
                            let _ = add_expense(&pool, uid, &store, date, amt, tag_id);
                            handle_successful_login(&pool, uid).await.0
                        },
                        TransactionsLoaded,
                    );
                }
            }
        ConfirmAddIncome => {
                if let Some(uid) = app.user_id {
                    let src = app.income_source.clone();
                    let date = chrono::Local::now().naive_local();
                    let amt = app.income_sum.parse().unwrap_or(0.0);
                    let pool = app.pool.clone();

                    app.clear_income_form();

                    return Command::perform(
                        async move {
                            let _ = add_income(&pool, uid, &src, date, amt);
                            handle_successful_login(&pool, uid).await.0
                        },
                        TransactionsLoaded,
                    );
                }
            }
        TransactionsLoaded(txs) => app.transactions = txs,
        ExitPressed => std::process::exit(0),
        SortTypeChanged(sort) => {
                app.sort_type = sort;
            },
        SubmitPasswordReset => {
            if app.new_password != app.confirm_new_password {
            app.reg_message = "Passwords do not match".into();
            } else {
            let pool   = app.pool.clone();
            let user   = app.login_username.clone();
            let secret = app.secret_pass.clone();
            let new_pw = app.new_password.clone();

            return Command::perform(
                    async move {
                        attempt_password_reset(&pool, &user, &secret, &new_pw).await
                    },
                    Message::PasswordResetResult,
                );
            }
        }
       

        ChangeIncomeDate(v) => {
            if let Ok(parsed) = NaiveDate::parse_from_str(&v, "%Y-%m-%d") {
                app.income_date = parsed.and_hms_opt(0, 0, 0).unwrap();
            }
            app.income_date_str = v;
        },

        ChangeExpenseDateString(v) => {
            if let Ok(parsed) = NaiveDate::parse_from_str(&v, "%Y-%m-%d") {
                app.expense_date = parsed.and_hms_opt(0, 0, 0).unwrap();
            }
            app.expense_date_str = v;
        },

        SetExpenseDateToToday => {
            let today = chrono::Local::now().naive_local().date().and_hms_opt(0, 0, 0).unwrap();
            app.expense_date = today;
            app.expense_date_str = today.format("%Y-%m-%d").to_string();
        }

    }

    Command::none()
}
