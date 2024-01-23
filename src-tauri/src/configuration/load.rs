use crate::configuration::model::ApplicationSettings;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn load_settings_from_file(file_path: &str) -> Option<ApplicationSettings> {
    if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path).expect("Unable to read settings file");

        let json: ApplicationSettings =
            serde_json::from_str(data.as_str()).expect("JSON was not well-formatted");

        return Some(json);
    }

    None
}

pub fn save_default_settings_to_file(file_path: &str) -> Option<ApplicationSettings> {
    let settings = ApplicationSettings::default();
    let file = File::create(file_path);
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &settings).expect("Settings file couldn't be saved");

    Some(settings)
}
