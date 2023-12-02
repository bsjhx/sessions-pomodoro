use uuid::Uuid;

pub struct SessionsCycle {
    id: Uuid,
    pub last_state_change: u64,
}

impl SessionsCycle {
    pub fn new() -> Self {
        SessionsCycle {
            id: Uuid::new_v4(),
            last_state_change: 0,
        }
    }
}
