// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        title -> Varchar,
        contents -> Varchar,
        completed -> Bool,
        date_created -> Date,
    }
}
