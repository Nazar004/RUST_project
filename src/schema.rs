use diesel::prelude::*;

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}


table! {
    transactions (tran_type, user_id, tran_id) {
        tran_type -> Varchar,
        user_id -> Int4,
        tran_id -> Int4,
        tran_source -> Varchar,
        date -> Varchar,
        tran_amount -> Float8,
        tran_comment -> Nullable<Varchar>,

    }
}


joinable!(transactions -> users (user_id));
allow_tables_to_appear_in_same_query!(users, transactions);
