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
    use assertor::{assert_that, EqualityAssertion, VecAssertion};
    use chrono::{DateTime, Duration, Utc};

    #[test]
    fn should_process_simple_work_cycle() {
        let now = Utc::now();
        let mut history_states = vec![];
        history_states.push(get_state(WorkingTimeState::ID, &now, 0));
        history_states.push(get_state(ShortBreakTimeState::ID, &now, 10));
        history_states.push(get_state(WorkingTimeState::ID, &now, 15));
        history_states.push(get_state(ShortBreakTimeState::ID, &now, 45));
        history_states.push(get_state(WorkingTimeState::ID, &now, 60));
        history_states.push(get_state(NothingState::ID, &now, 90));

        let actual = calculate(&history_states);
        let expected = get_expected(now);

        assert_eq!(actual, expected);
    }

    fn get_expected(now: DateTime<Utc>) -> StateStatisticsDetails {
        StateStatisticsDetails::new(
            90,
            vec![
                StateStatistics::new(WorkingTimeState::ID, now, now + Duration::seconds(10), 10),
                StateStatistics::new(
                    ShortBreakTimeState::ID,
                    now + Duration::seconds(10),
                    now + Duration::seconds(15),
                    5,
                ),
                StateStatistics::new(
                    WorkingTimeState::ID,
                    now + Duration::seconds(15),
                    now + Duration::seconds(45),
                    30,
                ),
                StateStatistics::new(
                    ShortBreakTimeState::ID,
                    now + Duration::seconds(45),
                    now + Duration::seconds(60),
                    15,
                ),
                StateStatistics::new(
                    WorkingTimeState::ID,
                    now + Duration::seconds(60),
                    now + Duration::seconds(90),
                    30,
                ),
            ],
        )
    }

    #[test]
    fn empty_array_should_be_empty() {
        let result = calculate(&vec![]);

        assert_that!(result.states).is_empty();
        assert_that!(result.total_length_in_minutes).is_equal_to(0);
    }

    fn get_state(id: &str, date: &DateTime<Utc>, seconds: i64) -> StateHistoryItem {
        let new_date = *date + Duration::seconds(seconds);
        StateHistoryItem::new(id, new_date)
    }
}
