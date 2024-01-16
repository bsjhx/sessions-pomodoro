// @generated automatically by Diesel CLI.
// TODO move this. Remember to change in diesel.toml
diesel::table! {
    states (id) {
        id -> Nullable<Integer>,
        state_id -> Nullable<Text>,
        started_time -> Nullable<Timestamp>,
    }
}
