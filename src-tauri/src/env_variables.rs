use std::env;

pub const POMODORO_DEVTOOLS_ENABLED: &str = "POMODORO_DEVTOOLS_ENABLED";
pub const POMODORO_ENABLE_TEST_DATA: &str = "POMODORO_ENABLE_TEST_DATA";

pub fn read_boolean_variable(flag_name: &str) -> bool {
    env::var(flag_name.to_string())
        .unwrap_or_default()
        .eq_ignore_ascii_case("true")
}
