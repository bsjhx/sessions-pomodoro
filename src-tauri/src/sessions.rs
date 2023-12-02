struct ApplicationContext {
    sessions_cycle: SessionsCycle,
    state: Box<dyn State>,
}

struct SessionsCycle {}

pub struct StoppedState;
pub struct PausedState;
pub struct PlayingState;

pub trait State {
    fn start_cycle(self: Box<Self>, player: &mut SessionsCycle) -> Box<dyn State>;
    fn finish_cycle(self: Box<Self>, player: &mut SessionsCycle) -> Box<dyn State>;

    fn end(self: Box<Self>, player: &mut SessionsCycle) -> Box<dyn State>;
}
