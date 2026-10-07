use chrono::{Local};
use leptos::{prelude::*};

use crate::{ActivityLogRange, get_current_week_start, presentation::text_formattings::format_hours_minutes};

#[component]
pub fn Statistics(
    logged_activities_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    view! {
        <aside class="statistics">
            {
                move || {
                    let today = Local::now().date_naive();
                    let current_week_start = get_current_week_start();

                    let (today_duration, week_duration, best_duration) = logged_activities_by_day.with(|logs| {
                        logs.as_ref().map_or((0, 0, 0), |logs| {
                            (
                                logs.activity_duration(today).unwrap_or_default(),
                                logs.duration_between(current_week_start, today),
                                logs.max_duration()
                            )
                        })
                    });
                    
                    let today_text = format_hours_minutes(today_duration);
                    let week_text = format_hours_minutes(week_duration);
                    let best_day_text = format_hours_minutes(best_duration);

                    view! {
                        <StatisticCard title="TODAY" value=today_text/>
                        <StatisticCard title="THIS WEEK" value=week_text/>
                        <StatisticCard title="BEST DAY" value=best_day_text/>
                    }
                }
            }
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