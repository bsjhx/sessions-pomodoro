use std::env;

pub const POMODORO_DEVTOOLS_ENABLED: &str = "POMODORO_DEVTOOLS_ENABLED";
pub const POMODORO_ENABLE_TEST_DATA: &str = "POMODORO_ENABLE_TEST_DATA";

pub fn read_boolean_variable(flag_name: &str) -> bool {
    let enabled_devtools = env::var(flag_name).unwrap_or_default();
    enabled_devtools == "true"
}
