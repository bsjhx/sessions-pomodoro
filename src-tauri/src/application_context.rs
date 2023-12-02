use crate::sessions::{SessionsCycle, State};

struct ApplicationContext {
    sessions_cycle: SessionsCycle,
    state: Box<dyn State>,
}
