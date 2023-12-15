use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct WorkCycle {
    id: Uuid,
    pub last_state_change: u64,
}

impl WorkCycle {
    pub fn new() -> Self {
        WorkCycle {
            id: Uuid::new_v4(),
            last_state_change: 0,
        }
    }

    pub fn update_last_state_change(&mut self) {
        self.last_state_change = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    }
}
