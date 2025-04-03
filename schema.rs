use diesel::prelude::*;

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}
