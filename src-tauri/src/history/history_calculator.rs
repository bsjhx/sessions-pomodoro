use crate::db::StateHistoryItem;
use crate::history::history_context::{StateStatistics, StateStatisticsDetails};
use crate::work_cycle::{NothingState, StateId};

pub fn calculate(history_states: &[StateHistoryItem]) -> StateStatisticsDetails {
    if history_states.len() == 1 {
        return StateStatisticsDetails::new(0, vec![]);
    }
    let mut states: Vec<StateStatistics> = vec![];
    let mut sum = 0;

    for (i, state) in history_states.iter().enumerate() {
        if i == history_states.len() - 1 {
            // todo apply last state which is not nothing state;
            return StateStatisticsDetails::new(sum, states);
        }
        if !is_nothing_state(state) {
            let started = *state.get_started_time();
            let finished = *history_states[i + 1].get_started_time();
            let diff = finished.signed_duration_since(started).num_seconds();
            sum += diff;
            states.push(StateStatistics::new(
                state.get_id(),
                started,
                finished,
                diff,
            ));
        }
    }

    StateStatisticsDetails::new(sum, states)
}

fn is_nothing_state(state: &StateHistoryItem) -> bool {
    state.get_id() == NothingState::ID
}

#[cfg(test)]
mod test {
    use crate::db::StateHistoryItem;
    use crate::history::history_calculator::calculate;
    use crate::history::history_context::{StateStatistics, StateStatisticsDetails};
    use crate::work_cycle::{NothingState, ShortBreakTimeState, StateId, WorkingTimeState};
    use assertor::{assert_that, EqualityAssertion};
    use chrono::{DateTime, Duration, Utc};

    #[test]
    fn empty_array_should_be_empty() {
        let result = calculate(&vec![]);
        assert_that!(result).is_equal_to(StateStatisticsDetails::new(0, vec![]));
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

        assert_that!(actual).is_equal_to(expected);
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
        let expected = get_single_work_cycle2(60, now);

        assert_that!(actual).is_equal_to(expected);
    }

    fn get_single_work_cycle(total_length: i64, now: DateTime<Utc>) -> StateStatisticsDetails {
        StateStatisticsDetails::new(
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

    fn get_single_work_cycle2(total_length: i64, now: DateTime<Utc>) -> StateStatisticsDetails {
        StateStatisticsDetails::new(
            total_length,
            vec![
                get_state_statistics(now, WorkingTimeState::ID, 0, 10),
                get_state_statistics(now, ShortBreakTimeState::ID, 10, 15),
                get_state_statistics(now, WorkingTimeState::ID, 15, 45),
                get_state_statistics(now, ShortBreakTimeState::ID, 45, 60),
            ],
        )
    }

    fn get_state_statistics(
        now: DateTime<Utc>,
        id: &str,
        seconds_started: i64,
        seconds_finished: i64,
    ) -> StateStatistics {
        StateStatistics::new(
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
