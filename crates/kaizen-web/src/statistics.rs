use chrono::{Local};
use leptos::{prelude::*};

use crate::{ActivityLogRange, get_current_week_start};

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
                        .and_then(|logs| logs.activity_duration(today))
                        .unwrap_or_default();

                    let today_text = format_duration(today_duration);

                    let current_week_start = get_current_week_start();
                    let duration = logged_activities_by_day
                        .get()
                        .map(|logs| logs.duration_between(current_week_start, today))
                        .unwrap_or_default();
                    
                    let week_text = format_duration(duration);

                    view! {
                        <StatisticCard title="TODAY" value=today_text/>
                        <StatisticCard title="THIS WEEK" value=week_text/>
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

pub fn format_duration(duration: u64) -> String {
    let minutes = (duration % 3600) / 60;
    let hours = duration / 3600;

    match (hours, minutes) {
        (0, minutes) => format!("{minutes}m"),
        _ => format!("{hours}h {minutes}m")
    }
}