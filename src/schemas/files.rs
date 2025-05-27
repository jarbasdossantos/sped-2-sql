// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        name -> Nullable<Text>,
    }
}
