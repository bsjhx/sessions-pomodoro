use app::configuration::WorkCycleSettings;
use app::work_cycle::application_context::ApplicationContext;
use app::work_cycle::{LongBreakTimeState, NothingState, WorkingTimeState};
use app::work_cycle::{ShortBreakTimeState, StateId};

#[test]
fn application_context_current_state_should_be_ok() {
    let mut settings = WorkCycleSettings::new();
    settings.work_sessions_to_long_break = 2;
    let mut application_context = ApplicationContext::new(settings);

    // init testable db

    assert_eq!(
        application_context.get_current_state_name(),
        NothingState::ID
    );

    application_context.start_cycle();
    assert_eq!(
        application_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        ShortBreakTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        LongBreakTimeState::ID
    );

    application_context.end_current_session();
    assert_eq!(
        application_context.get_current_state_name(),
        WorkingTimeState::ID
    );

    application_context.finish_cycle();
    assert_eq!(
        application_context.get_current_state_name(),
        NothingState::ID
    );
}

#[test]
fn application_context_should_keep_history() {
    let mut settings = WorkCycleSettings::new();
    settings.work_sessions_to_long_break = 3;
    let mut application_context = ApplicationContext::new(settings);

    let mut states_expected = vec![];
    states_expected.push(WorkingTimeState::ID);
    states_expected.push(ShortBreakTimeState::ID);
    states_expected.push(WorkingTimeState::ID);
    states_expected.push(ShortBreakTimeState::ID);
    states_expected.push(WorkingTimeState::ID);
    states_expected.push(LongBreakTimeState::ID);
    states_expected.push(NothingState::ID);

    application_context.start_cycle();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.end_current_session();
    application_context.finish_cycle();

    for (i, actual_state) in application_context.get_current_history().iter().enumerate() {
        assert_eq!(actual_state.get_name(), states_expected[i]);
    }
}
