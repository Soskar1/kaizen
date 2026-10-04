use chrono::Local;
use leptos::{prelude::*};

use crate::ActivityLogRange;

#[component]
pub fn Staticstics(
    logged_activites_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    view! {
        <aside class="statistics">
            {
                move || {
                    let today = Local::now().date_naive();
                    let today_duration = logged_activites_by_day
                        .get()
                        .and_then(|logs| logs.activity_duration(today))
                        .unwrap_or_default();

                    let text = get_hours_minutes_time(today_duration);

                    view! {
                        <StatisticCard title="TODAY" value=text/>
                    }
                }
            }
            
            <StatisticCard title="THIS WEEK" value="5h 20m"/>
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