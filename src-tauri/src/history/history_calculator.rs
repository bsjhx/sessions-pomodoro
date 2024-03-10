use crate::db::StateHistoryItem;
use crate::history::history_context::{
    RunningStateDetails, StateDurationDetails, StatesDurationsDetails,
};
use crate::work_cycle::{NothingState, StateId};

pub fn calculate(history_states: &[StateHistoryItem]) -> StatesDurationsDetails {
    if history_states.len() == 1 {
        let state = history_states.first().unwrap();
        return StatesDurationsDetails::new_with_running_state(
            0,
            vec![],
            RunningStateDetails::new(state.get_id(), *state.get_started_time()),
        );
    }

    let mut states: Vec<StateDurationDetails> = vec![];
    let mut sum = 0;

    for (i, state) in history_states.iter().enumerate() {
        if i == history_states.len() - 1 {
            return if !is_nothing_state(state) {
                StatesDurationsDetails::new_with_running_state(
                    sum,
                    states,
                    RunningStateDetails::new(state.get_id(), *state.get_started_time()),
                )
            } else {
                StatesDurationsDetails::new(sum, states)
            };
        }

        if !is_nothing_state(state) {
            let started = *state.get_started_time();
            let finished = *history_states[i + 1].get_started_time();
            let diff = finished.signed_duration_since(started).num_seconds();
            sum += diff;
            states.push(StateDurationDetails::new(
                state.get_id(),
                started,
                finished,
                diff,
            ));
        }
    }

    StatesDurationsDetails::new(sum, states)
}

fn is_nothing_state(state: &StateHistoryItem) -> bool {
    state.get_id() == NothingState::ID
}

#[cfg(test)]
mod test {
    use crate::db::StateHistoryItem;
    use crate::history::history_calculator::calculate;
    use crate::history::history_context::{
        RunningStateDetails, StateDurationDetails, StatesDurationsDetails,
    };
    use crate::work_cycle::{NothingState, ShortBreakTimeState, StateId, WorkingTimeState};
    use assertor::{assert_that, EqualityAssertion};
    use chrono::{DateTime, Duration, Utc};

    #[test]
    fn empty_array_should_be_empty() {
        let result = calculate(&vec![]);
        assert_that!(result).is_equal_to(StatesDurationsDetails::new(0, vec![]));
    }

    #[test]
    fn should_process_single_finished_work_cycle() {
        let now = Utc::now();
        let mut history_states = vec![];
        history_states.push(get_state(WorkingTimeState::ID, &now, 0));
        history_states.push(get_state(ShortBreakTimeState::ID, &now, 10));
        history_states.push(get_state(WorkingTimeState::ID, &now, 15));
        history_states.push(get_state(ShortBreakTimeState::ID, &now, 45));
        history_states.push(get_state(WorkingTimeState::ID, &now, 60));
        history_states.push(get_state(NothingState::ID, &now, 90));

        let actual = calculate(&history_states);
        let expected = get_single_work_cycle(90, now);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_process_not_finished_work_cycle() {
        let now = Utc::now();
        let mut history_states = vec![];
        history_states.push(get_state(WorkingTimeState::ID, &now, 0));
        history_states.push(get_state(ShortBreakTimeState::ID, &now, 10));
        history_states.push(get_state(WorkingTimeState::ID, &now, 15));
        history_states.push(get_state(ShortBreakTimeState::ID, &now, 45));
        history_states.push(get_state(WorkingTimeState::ID, &now, 60));

        let actual = calculate(&history_states);
        let expected = get_single_work_cycle_unfinished(60, now);

        assert_eq!(actual, expected);
    }

    fn get_single_work_cycle(total_length: i64, now: DateTime<Utc>) -> StatesDurationsDetails {
        StatesDurationsDetails::new(
            total_length,
            vec![
                get_state_statistics(now, WorkingTimeState::ID, 0, 10),
                get_state_statistics(now, ShortBreakTimeState::ID, 10, 15),
                get_state_statistics(now, WorkingTimeState::ID, 15, 45),
                get_state_statistics(now, ShortBreakTimeState::ID, 45, 60),
                get_state_statistics(now, WorkingTimeState::ID, 60, 90),
            ],
        )
    }

    fn get_single_work_cycle_unfinished(
        total_length: i64,
        now: DateTime<Utc>,
    ) -> StatesDurationsDetails {
        StatesDurationsDetails::new_with_running_state(
            total_length,
            vec![
                get_state_statistics(now, WorkingTimeState::ID, 0, 10),
                get_state_statistics(now, ShortBreakTimeState::ID, 10, 15),
                get_state_statistics(now, WorkingTimeState::ID, 15, 45),
                get_state_statistics(now, ShortBreakTimeState::ID, 45, 60),
            ],
            RunningStateDetails::new(WorkingTimeState::ID, now + Duration::seconds(60)),
        )
    }

    fn get_state_statistics(
        now: DateTime<Utc>,
        id: &str,
        seconds_started: i64,
        seconds_finished: i64,
    ) -> StateDurationDetails {
        StateDurationDetails::new(
            id,
            now + Duration::seconds(seconds_started),
            now + Duration::seconds(seconds_finished),
            seconds_finished - seconds_started,
        )
    }

    fn get_state(id: &str, date: &DateTime<Utc>, seconds: i64) -> StateHistoryItem {
        let new_date = *date + Duration::seconds(seconds);
        StateHistoryItem::new(id, new_date)
    }
}
