use chrono::NaiveDate;
use kaizen_core::streak::{week_start, week_streak};

use crate::domain::activity_log_range::ActivityLogRange;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct ActivityStreak {
    current: u32,
    current_week_logged: bool
}

impl ActivityStreak {
    pub fn new(logs: &ActivityLogRange, today: NaiveDate) -> Self {
        Self {
            current: week_streak(&logs.day_logs(), today),
            current_week_logged: logs.duration_between(week_start(today), today) > 0
        }
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn current_week_logged(&self) -> bool {
        self.current_week_logged
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rstest::rstest;
    use super::*;

    // 2026-10-15 is a Thursday; its week starts on 2026-10-12
    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, 15).unwrap()
    }

    fn date(month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, month, day).unwrap()
    }

    fn logs(dates: &[NaiveDate]) -> ActivityLogRange {
        ActivityLogRange::new(
            date(1, 1),
            date(12, 31),
            dates.iter().map(|date| (*date, 60)).collect::<HashMap<_, _>>()
        )
    }

    #[rstest]
    #[case::no_logs(vec![], 0, false)]
    #[case::logged_this_week(vec![date(10, 5), date(10, 13)], 2, true)]
    #[case::logged_last_week_only(vec![date(10, 5)], 1, false)]
    #[case::streak_broken(vec![date(9, 28)], 0, false)]
    fn new_computes_streak_and_current_week(
        #[case] dates: Vec<NaiveDate>,
        #[case] expected_current: u32,
        #[case] expected_current_week_logged: bool
    ) {
        // Arrange
        let logs = logs(&dates);

        // Act
        let result = ActivityStreak::new(&logs, today());

        // Assert
        assert_eq!(result.current(), expected_current);
        assert_eq!(result.current_week_logged(), expected_current_week_logged);
    }
}
