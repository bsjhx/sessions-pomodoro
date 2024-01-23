use app::settings;
use assertor::{assert_that, BooleanAssertion, EqualityAssertion};
use random_string::generate;
use std::fs;
use std::path::Path;

#[test]
fn default_settings_should_be_saved_file() {
    // Given
    let test_settings_path = get_random_file_name();
    let test_settings_path = test_settings_path.as_str();

    // File does not exists
    assert_that!(Path::new(test_settings_path).exists()).is_false();

    // When saving default settings to file
    let actual_default_settings = settings::save_default_settings_to_file(test_settings_path);
    assert_that!(actual_default_settings.is_some()).is_true();

    // Then settings are loaded and file is created
    assert_that!(Path::new(test_settings_path).exists()).is_true();

    let actual_default_settings = actual_default_settings.unwrap();
    assert_that!(actual_default_settings
        .db_file_path
        .ends_with(".config/sessions-pomodoro/database.sqlite"))
    .is_true();
    let wcs = actual_default_settings.work_cycle_settings;
    assert_that!(wcs.work_sessions_to_long_break).is_equal_to(3);
    assert_that!(wcs.time_settings.working_time).is_equal_to(25 * 60);
    assert_that!(wcs.time_settings.short_break_time).is_equal_to(5 * 60);
    assert_that!(wcs.time_settings.long_break_time).is_equal_to(15 * 60);

    // Remove test settings file
    fs::remove_file(test_settings_path).expect("Test settings file failed to remove");
}

#[test]
fn settings_should_be_loaded_from_file() {
    // Given
    let test_settings_path = get_random_file_name();
    let test_settings_path = test_settings_path.as_str();

    // File does not exists
    assert_that!(Path::new(test_settings_path).exists()).is_false();

    // Some default settings are stored
    let _ = settings::save_default_settings_to_file(test_settings_path);

    // When loading settings from file
    let actual_settings = settings::load_settings_from_file(test_settings_path);

    // Then
    assert_that!(actual_settings.is_some()).is_true();
    let actual_default_settings = actual_settings.unwrap();

    assert_that!(actual_default_settings
        .db_file_path
        .ends_with(".config/sessions-pomodoro/database.sqlite"))
    .is_true();
    let wcs = actual_default_settings.work_cycle_settings;
    assert_that!(wcs.work_sessions_to_long_break).is_equal_to(3);
    assert_that!(wcs.time_settings.working_time).is_equal_to(25 * 60);
    assert_that!(wcs.time_settings.short_break_time).is_equal_to(5 * 60);
    assert_that!(wcs.time_settings.long_break_time).is_equal_to(15 * 60);

    // Remove test settings file
    fs::remove_file(test_settings_path).expect("Test settings file failed to remove");
}

fn get_random_file_name() -> String {
    let charset = ('a'..'z').into_iter().collect::<String>();
    let test_settings_path = format!("./{}.json", generate(10, charset));
    test_settings_path
}
