// @generated automatically by Diesel CLI.

diesel::table! {
    kvs (k) {
        k -> Integer,
        v -> Text,
    }
}
