use chrono::{Datelike, Duration, Local, NaiveDate};
use kaizen_core::streak::{longest_week_streak, week_start, week_streak};
use leptos::prelude::*;

use crate::ActivityLogRange;

const WEEKDAY_LABELS: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];
const MILESTONE_STEP: u32 = 10;

#[derive(Clone, PartialEq)]
struct StreakSummary {
    today: NaiveDate,
    current: u32,
    best: u32,
    logged_week_days: [bool; 7]
}

impl StreakSummary {
    fn new(logs: Option<&ActivityLogRange>, today: NaiveDate) -> Self {
        let Some(logs) = logs else {
            return Self { today, current: 0, best: 0, logged_week_days: [false; 7] };
        };

        let day_logs = logs.day_logs();
        let current_week_start = week_start(today);
        let logged_week_days = std::array::from_fn(|offset| {
            let day = current_week_start + Duration::days(offset as i64);
            logs.activity_duration(day).unwrap_or_default() > 0
        });

        Self {
            today,
            current: week_streak(&day_logs, today),
            best: longest_week_streak(&day_logs, today),
            logged_week_days
        }
    }

    fn current_week_logged(&self) -> bool {
        self.logged_week_days.iter().any(|&logged| logged)
    }
}

#[component]
pub fn StreakCard(
    #[prop(into)] selected_activity: ReadSignal<String>,
    logged_activites_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    let inactive = move || selected_activity.with(|activity| activity.is_empty());

    let summary = Memo::new(move |_| {
        let today = Local::now().date_naive();

        if inactive() {
            return StreakSummary::new(None, today);
        }

        logged_activites_by_day.with(|logs| StreakSummary::new(logs.as_ref(), today))
    });

    let next_milestone = move || next_milestone(summary.with(|summary| summary.current));

    view! {
        <article class="streak-card" class:inactive=inactive aria-disabled=move || inactive().to_string()>
            <div class="streak-summary" class:lit=move || summary.with(|summary| summary.current > 0)>
                <svg class="streak-flame" viewBox="0 0 32 36" aria-hidden="true">
                    <defs>
                        <linearGradient id="streak-flame-outer-gradient" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color="#ffc93c"/>
                            <stop offset="55%" stop-color="#ff7a1a"/>
                            <stop offset="100%" stop-color="#e8381c"/>
                        </linearGradient>
                        <linearGradient id="streak-flame-inner-gradient" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color="#fff7c7"/>
                            <stop offset="100%" stop-color="#ffd23f"/>
                        </linearGradient>
                    </defs>
                    <path
                        class="streak-flame-outer"
                        d="M16 1C18 8 29 13 29 23C29 30.5 23.2 35 16 35C8.8 35 3 30.5 3 23.5C3 17.5 6.5 13.5 10 11C10 15.5 11.5 18.5 14 19.5C12.5 13 13 6.5 16 1Z"
                    />
                    <path
                        class="streak-flame-inner"
                        d="M16 20.5C20 23.5 22 26 22 29C22 32 19.3 34 16 34C12.7 34 10 32 10 29C10 26 12 23.5 16 20.5Z"
                    />
                </svg>

                <strong class="streak-count">{move || summary.with(|summary| summary.current)}</strong>

                <div class="streak-title">
                    <span class="streak-title-main">"week streak"</span>
                    <span class="streak-title-sub">
                        {move || {
                            if inactive() {
                                String::from("pick an activity to start")
                            } else {
                                format!("best {}", summary.with(|summary| summary.best))
                            }
                        }}
                    </span>
                </div>
            </div>

            <ol class="streak-week" aria-label="This week">
                {move || summary.with(|summary| {
                    let today_index = summary.today.weekday().num_days_from_monday() as usize;

                    WEEKDAY_LABELS
                        .iter()
                        .enumerate()
                        .map(|(index, label)| view! {
                            <StreakDay
                                label=*label
                                done=summary.logged_week_days[index]
                                today=index == today_index
                            />
                        })
                        .collect_view()
                })}
            </ol>

            <div class="streak-goal">
                <div class="streak-goal-header">
                    <span class="streak-goal-title">
                        {move || {
                            let weeks_left = next_milestone() - summary.with(|summary| summary.current);
                            format!("{} to {}", weeks_text(weeks_left), next_milestone())
                        }}
                    </span>
                    <span class="streak-goal-count">
                        {move || format!("{} / {}", summary.with(|summary| summary.current), next_milestone())}
                    </span>
                </div>

                <div
                    class="streak-progress"
                    role="progressbar"
                    aria-valuemin="0"
                    aria-valuemax=next_milestone
                    aria-valuenow=move || summary.with(|summary| summary.current)
                >
                    <div
                        class="streak-progress-fill"
                        style=move || {
                            let progress = summary.with(|summary| summary.current) * 100 / next_milestone();
                            format!("width: {progress}%")
                        }
                    ></div>
                </div>

                <span class="streak-warning">
                    {move || {
                        if inactive() {
                            return String::from("No activity selected");
                        }

                        summary.with(|summary| {
                            if summary.current_week_logged() {
                                String::from("This week is done · keep it going")
                            } else {
                                match days_left_in_week(summary.today) {
                                    1 => String::from("Nothing logged this week · last day"),
                                    days => format!("Nothing logged this week · {days} days left")
                                }
                            }
                        })
                    }}
                </span>
            </div>
        </article>
    }
}

#[component]
fn StreakDay(
    #[prop(into)] label: String,
    #[prop(optional)] done: bool,
    #[prop(optional)] today: bool
) -> impl IntoView {
    view! {
        <li class="streak-day" class:done=done class:today=today>
            <span class="streak-day-circle">
                {done.then(|| view! {
                    <svg viewBox="0 0 16 16" aria-hidden="true">
                        <path d="M3.5 8.5L6.5 11.5L12.5 5" />
                    </svg>
                })}
            </span>
            <span class="streak-day-label">{label}</span>
        </li>
    }
}

fn next_milestone(streak: u32) -> u32 {
    (streak / MILESTONE_STEP + 1) * MILESTONE_STEP
}

fn days_left_in_week(today: NaiveDate) -> u32 {
    7 - today.weekday().num_days_from_monday()
}

fn weeks_text(weeks: u32) -> String {
    match weeks {
        1 => String::from("1 week"),
        weeks => format!("{weeks} weeks")
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case::no_streak(0, 10)]
    #[case::below_milestone(9, 10)]
    #[case::reached_milestone(10, 20)]
    #[case::large_streak(41, 50)]
    fn next_milestone_returns_next_multiple_of_ten(#[case] streak: u32, #[case] expected_milestone: u32) {
        // Arrange & Act
        let result = next_milestone(streak);

        // Assert
        assert_eq!(result, expected_milestone);
    }

    #[rstest]
    #[case::monday(12, 7)]
    #[case::thursday(15, 4)]
    #[case::sunday(18, 1)]
    fn days_left_in_week_includes_today(#[case] day: u32, #[case] expected_days: u32) {
        // Arrange
        let today = NaiveDate::from_ymd_opt(2026, 10, day).unwrap();

        // Act
        let result = days_left_in_week(today);

        // Assert
        assert_eq!(result, expected_days);
    }

    #[rstest]
    #[case::single(1, "1 week")]
    #[case::plural(9, "9 weeks")]
    fn weeks_text_pluralizes(#[case] weeks: u32, #[case] expected_text: &str) {
        // Arrange & Act
        let result = weeks_text(weeks);

        // Assert
        assert_eq!(result, expected_text);
    }
}
