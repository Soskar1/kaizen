use chrono::{Datelike, Duration, Local};
use leptos::{prelude::*};

use crate::ActivityLogRange;

#[component]
pub fn Statistics(
    logged_activities_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    view! {
        <aside class="statistics">
            {
                move || {
                    let today = Local::now().date_naive();
                    let today_duration = logged_activities_by_day
                        .get()
                        .and_then(|logs| logs.get_activity_duration(today))
                        .unwrap_or_default();

                    let text = get_hours_minutes_time(today_duration);

                    view! {
                        <StatisticCard title="TODAY" value=text/>
                    }
                }
            }
            
            {
                move || {
                    let today = Local::now().date_naive();
                    let current_week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
                    let duration = logged_activities_by_day
                        .get()
                        .map(|logs| logs.get_activity_duration_from_range(current_week_start, today))
                        .unwrap_or_default();
                    
                    let text = get_hours_minutes_time(duration);

                    view! {
                        <StatisticCard title="THIS WEEK" value=text/>
                    }
                }
            }

            <StatisticCard title="BEST DAY" value="3h 10m"/>
        </aside>
    }
}

#[component]
fn StatisticCard(
    #[prop(into)] title: String,
    #[prop(into)] value: String
) -> impl IntoView {
    view! {
        <article class="stat-card">
            <span class="label">{title}</span>
            <strong>{value}</strong>
        </article>
    }
}

pub fn get_hours_minutes_time(duration: u64) -> String {
    let minutes = (duration % 3600) / 60;
    let hours = duration / 3600;
    format!("{hours}h {minutes}m")
}