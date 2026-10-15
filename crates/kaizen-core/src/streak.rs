use std::collections::HashSet;
use chrono::{Datelike, Duration, NaiveDate};
use crate::day_log::DayLog;

pub fn week_streak<'a>(day_logs: impl IntoIterator<Item = &'a DayLog>, today: NaiveDate) -> u32 {
    let logged_weeks = logged_weeks(day_logs, today);

    let mut week = week_start(today);

    if !logged_weeks.contains(&week) {
        week -= Duration::weeks(1);
    }

    let mut streak = 0;

    while logged_weeks.contains(&week) {
        streak += 1;
        week -= Duration::weeks(1);
    }

    streak
}

pub fn longest_week_streak<'a>(day_logs: impl IntoIterator<Item = &'a DayLog>, today: NaiveDate) -> u32 {
    let mut logged_weeks: Vec<NaiveDate> = logged_weeks(day_logs, today).into_iter().collect();
    logged_weeks.sort();

    let mut longest = 0;
    let mut current = 0;
    let mut previous_week: Option<NaiveDate> = None;

    for week in logged_weeks {
        current = match previous_week {
            Some(previous) if week - previous == Duration::weeks(1) => current + 1,
            _ => 1
        };

        longest = longest.max(current);
        previous_week = Some(week);
    }

    longest
}

pub fn week_start(date: NaiveDate) -> NaiveDate {
    date - Duration::days(date.weekday().num_days_from_monday() as i64)
}

fn logged_weeks<'a>(day_logs: impl IntoIterator<Item = &'a DayLog>, today: NaiveDate) -> HashSet<NaiveDate> {
    day_logs
        .into_iter()
        .filter(|day_log| day_log.activity_duration() > 0 && day_log.date() <= today)
        .map(|day_log| week_start(day_log.date()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // 2026-10-15 is a Thursday; its week runs from 2026-10-12 (Monday) to 2026-10-18 (Sunday)
    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 10, 15).unwrap()
    }

    fn day_log(month: u32, day: u32) -> DayLog {
        DayLog::new(NaiveDate::from_ymd_opt(2026, month, day).unwrap(), 60)
    }
    
    #[rstest]
    #[case::no_logs(vec![], 0)]
    #[case::logged_this_week(vec![day_log(10, 12)], 1)]
    #[case::logged_today(vec![day_log(10, 15)], 1)]
    #[case::logged_last_week_only(vec![day_log(10, 5)], 1)]
    #[case::logged_two_weeks_ago_only(vec![day_log(9, 30)], 0)]
    #[case::sunday_and_monday_are_different_weeks(vec![day_log(10, 11), day_log(10, 12)], 2)]
    #[case::multiple_logs_in_one_week(vec![day_log(10, 12), day_log(10, 13), day_log(10, 14)], 1)]
    #[case::consecutive_weeks(vec![day_log(9, 28), day_log(10, 10), day_log(10, 14)], 3)]
    #[case::gap_breaks_streak(vec![day_log(9, 21), day_log(10, 5), day_log(10, 13)], 2)]
    #[case::future_logs_ignored(vec![day_log(10, 17), day_log(10, 20)], 0)]
    fn week_streak_counts_consecutive_logged_weeks(#[case] day_logs: Vec<DayLog>, #[case] expected_streak: u32) {
        // Arrange & Act
        let result = week_streak(&day_logs, today());

        // Assert
        assert_eq!(result, expected_streak);
    }

    #[test]
    fn week_streak_ignores_empty_logs() {
        // Arrange
        let day_logs = vec![DayLog::new(today(), 0)];

        // Act
        let result = week_streak(&day_logs, today());

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn week_streak_spans_year_boundary() {
        // Arrange
        let today = NaiveDate::from_ymd_opt(2027, 1, 4).unwrap();
        let day_logs = vec![
            DayLog::new(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(), 60),
            DayLog::new(NaiveDate::from_ymd_opt(2027, 1, 4).unwrap(), 60),
        ];

        // Act
        let result = week_streak(&day_logs, today);

        // Assert
        assert_eq!(result, 2);
    }

    #[rstest]
    #[case::no_logs(vec![], 0)]
    #[case::single_week(vec![day_log(10, 12), day_log(10, 14)], 1)]
    #[case::current_streak_is_longest(vec![day_log(9, 28), day_log(10, 10), day_log(10, 14)], 3)]
    #[case::past_streak_is_longest(vec![day_log(9, 7), day_log(9, 14), day_log(9, 21), day_log(10, 5), day_log(10, 13)], 3)]
    #[case::broken_streak_still_counts(vec![day_log(8, 31), day_log(9, 7)], 2)]
    #[case::future_logs_ignored(vec![day_log(10, 13), day_log(10, 20), day_log(10, 27)], 1)]
    fn longest_week_streak_returns_longest_run(#[case] day_logs: Vec<DayLog>, #[case] expected_streak: u32) {
        // Arrange & Act
        let result = longest_week_streak(&day_logs, today());

        // Assert
        assert_eq!(result, expected_streak);
    }
}
