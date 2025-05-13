use diesel::prelude::*;

table! {
    transactions (tran_id) {
        tran_id -> Int4,
        tran_type -> Varchar,
        user_id -> Int4,
        tran_source -> Varchar,
        date -> Varchar,
        tran_amount -> Float8,
        tran_comment -> Nullable<Varchar>,
        tag_id -> Nullable<Int4>,
    }
}

table! {
    expense_tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        secret_pass ->Varchar,
    }
}

joinable!(transactions -> users (user_id));
joinable!(transactions -> expense_tags (tag_id));

allow_tables_to_appear_in_same_query!(
    transactions,
    users,
    expense_tags,
);

