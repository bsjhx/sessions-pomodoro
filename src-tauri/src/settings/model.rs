use serde::{Deserialize, Serialize};
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ApplicationSettings {
    pub work_cycle_settings: WorkCycleSettings,
    pub db_file_path: String,
}

#[derive(Serialize, Copy, Clone, Debug, Deserialize)]
pub struct WorkCycleSettings {
    #[serde(rename = "timeSettings")]
    pub time_settings: TimeSettings,
    #[serde(rename = "workSessionsToLongBreak")]
    pub work_sessions_to_long_break: u16,
}

#[derive(Serialize, Copy, Clone, Debug, Deserialize)]
pub struct TimeSettings {
    #[serde(rename = "workingTime")]
    pub working_time: i32,
    #[serde(rename = "shortBreakTime")]
    pub short_break_time: i32,
    #[serde(rename = "longBreakTime")]
    pub long_break_time: i32,
}

impl WorkCycleSettings {
    pub fn new() -> Self {
        WorkCycleSettings {
            time_settings: Default::default(),
            work_sessions_to_long_break: 0,
        }
    }
}

impl TimeSettings {
    pub fn new(working_time: i32, short_break_time: i32, long_break_time: i32) -> Self {
        TimeSettings {
            working_time,
            short_break_time,
            long_break_time,
        }
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.to_str().unwrap().to_string();

        ApplicationSettings {
            work_cycle_settings: Default::default(),
            db_file_path: format!("{}/.config/sessions-pomodoro/database.sqlite", home_dir),
        }
    }
}

impl Default for WorkCycleSettings {
    fn default() -> Self {
        WorkCycleSettings {
            time_settings: Default::default(),
            work_sessions_to_long_break: 3,
        }
    }
}

impl Default for TimeSettings {
    fn default() -> Self {
        TimeSettings::new(25 * 60, 5 * 60, 15 * 60)
    }
}
