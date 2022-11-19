// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Bpchar,
        description -> Bpchar,
        is_completed -> Bool,
    }
}
