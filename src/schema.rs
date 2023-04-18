// @generated automatically by Diesel CLI.

diesel::table! {
    cvs (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        author -> Text,
    }
}
