use crate::db::StateHistoryItem;
use crate::history::history_context::{StateStatistics, StateStatisticsDetails};
use crate::work_cycle::{NothingState, StateId, WorkingTimeState};

pub fn calculate(history_states: &Vec<StateHistoryItem>) -> StateStatisticsDetails {
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
                WorkingTimeState::ID,
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

        let result = calculate(&history_states);

        assert_that!(result.states.len()).is_equal_to(5);
        assert_that!(result.total_length_in_minutes).is_equal_to(90);
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
