use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub struct TimeSettings {
    #[serde(rename = "workingTime")]
    pub working_time: i32,
    #[serde(rename = "breakTime")]
    pub break_time: i32,
}

impl TimeSettings {
    pub fn new(working_time: i32, break_time: i32) -> Self {
        TimeSettings {
            working_time,
            break_time,
        }
    }
}
