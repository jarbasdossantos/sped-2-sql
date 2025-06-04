// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        name -> Text,
        sped_type -> Text
    }
}
