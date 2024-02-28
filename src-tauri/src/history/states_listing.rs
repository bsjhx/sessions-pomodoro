use chrono::{DateTime, Utc};

struct StateHistoryItem {
    id: String,
    time: DateTime<Utc>,
}
// SELECT id, state_id, started_time FROM states where started_time >= '2024-01-23 00:00:00' and started_time <= '2024-01-23 23:59:59';
// pub fn get_states_for_day() -> Vec {}
