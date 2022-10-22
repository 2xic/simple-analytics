// @generated automatically by Diesel CLI.

diesel::table! {
    analytics (id) {
        id -> Nullable<Integer>,
        user_agent -> Nullable<Text>,
        ip -> Nullable<Text>,
        metadata -> Nullable<Text>,
    }
}
