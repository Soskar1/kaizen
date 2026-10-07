use std::collections::HashMap;

use chrono::NaiveDate;

#[derive(Clone)]
pub struct ActivityLogRange {
    from: NaiveDate,
    to: NaiveDate,
    logs: HashMap<NaiveDate, u64>
}

impl ActivityLogRange {
    pub fn new(from: NaiveDate, to: NaiveDate, logs: HashMap<NaiveDate, u64>) -> Self {
        ActivityLogRange { from, to, logs }
    }

    pub fn from(&self) -> NaiveDate {
        self.from
    }

    pub fn to(&self) -> NaiveDate {
        self.to
    }

    pub fn activity_duration(&self, date: NaiveDate) -> Option<u64> {
        self.logs.get(&date).copied()
    }

    pub fn duration_between(&self, from: NaiveDate, to: NaiveDate) -> u64 {
        if from > to {
            return 0;
        }

        self.logs
            .iter()
            .filter(|(date, _)| **date >= from && **date <= to)
            .map(|(_, duration)| *duration)
            .sum()
    }

    pub fn duration_sum(&self) -> u64 {
        self.logs.values().sum()
    }

    pub fn max_duration(&self) -> u64 {
        self.logs
            .values()
            .copied()
            .max()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 9, day).unwrap()
    }

    fn create_activity_log_range() -> ActivityLogRange {
        ActivityLogRange {
            from: date(20),
            to: date(25),
            logs: HashMap::from([
                (date(20), 60),
                (date(21), 120),
                // September 22 intentionally has no log.
                (date(23), 180),
                (date(25), 240),
            ]),
        }
    }

    #[test]
    fn from_returns_start_date() {
        // Arrange
        let logs = create_activity_log_range();

        // Act
        let result = logs.from();

        // Assert
        assert_eq!(result, date(20));
    }

    #[test]
    fn to_returns_end_date() {
        // Arrange
        let logs = create_activity_log_range();

        // Act
        let result = logs.to();

        // Assert
        assert_eq!(result, date(25));
    }

    #[test]
    fn activity_duration_returns_duration_for_existing_date() {
        // Arrange
        let logs = create_activity_log_range();

        // Act
        let result = logs.activity_duration(date(21));

        // Assert
        assert_eq!(result, Some(120));
    }

    #[test]
    fn activity_duration_returns_none_for_missing_date() {
        // Arrange
        let logs = create_activity_log_range();

        // Act
        let result = logs.activity_duration(date(22));

        // Assert
        assert_eq!(result, None);
    }

    #[rstest]
    #[case::sums_logs_in_range(21, 23, 300)]
    #[case::includes_both_boundaries(20, 21, 180)]
    #[case::single_day_range(21, 21, 120)]
    #[case::reversed_range(23, 20, 0)]
    #[case::range_without_logs(22, 22, 0)]
    #[case::complete_range(20, 25, 600)]
    fn duration_between_returns_expected_duration(
        #[case] from_day: u32,
        #[case] to_day: u32,
        #[case] expected_duration: u64,
    ) {
        // Arrange
        let logs = create_activity_log_range();

        // Act
        let result = logs.duration_between(
            date(from_day),
            date(to_day),
        );

        // Assert
        assert_eq!(result, expected_duration);
    }

    #[test]
    fn duration_sum_returns_sum_of_all_logs() {
        // Arrange
        let logs = create_activity_log_range();

        // Act
        let result = logs.duration_sum();

        // Assert
        assert_eq!(result, 600);
    }

    #[test]
    fn duration_sum_returns_zero_when_logs_are_empty() {
        // Arrange
        let logs = ActivityLogRange {
            from: date(20),
            to: date(25),
            logs: HashMap::new(),
        };

        // Act
        let result = logs.duration_sum();

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn max_duration_returns_largest_duration() {
        // Arrange
        let logs = create_activity_log_range();

        // Act
        let result = logs.max_duration();

        // Assert
        assert_eq!(result, 240);
    }

    #[test]
    fn max_duration_returns_zero_when_logs_are_empty() {
        // Arrange
        let logs = ActivityLogRange {
            from: date(20),
            to: date(25),
            logs: HashMap::new(),
        };

        // Act
        let result = logs.max_duration();

        // Assert
        assert_eq!(result, 0);
    }
}