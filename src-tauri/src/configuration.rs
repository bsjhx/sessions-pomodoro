use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub struct TimeSettings {
    #[serde(rename = "workingTime")]
    pub working_time: i32,
    #[serde(rename = "breakTime")]
    pub break_time: i32,
}
