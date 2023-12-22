use serde::{Deserialize, Serialize};

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

impl Default for TimeSettings {
    fn default() -> Self {
        TimeSettings::new(0, 0, 0)
    }
}

impl Default for WorkCycleSettings {
    fn default() -> Self {
        WorkCycleSettings {
            time_settings: Default::default(),
            work_sessions_to_long_break: 0,
        }
    }
}
