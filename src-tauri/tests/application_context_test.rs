use app::work_cycle::application_context::ApplicationContext;
use app::work_cycle::NothingState;
use app::work_cycle::StateId;

#[test]
fn application_context_should_keep_states_history() {
    let application_context = ApplicationContext::new();
    assert_eq!(
        application_context.get_current_state_name(),
        NothingState::ID
    );
}
