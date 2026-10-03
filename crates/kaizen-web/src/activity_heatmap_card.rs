use chrono::{Duration, Local, NaiveDate};
use leptos::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct DayActivity {
    duration_in_seconds: u64,
    date: NaiveDate
}

#[component]
pub fn ActivityHeatmapCard() -> impl IntoView {
    view! {
        <section class="heatmap-card">
            <header class="heatmap-card-header">
                <div class="heatmap-legend" aria-label="Activity level legend">
                    <span>"Less"</span>

                    <span class="legend-day level-0"></span>
                    <span class="legend-day level-1"></span>
                    <span class="legend-day level-2"></span>
                    <span class="legend-day level-3"></span>
                    <span class="legend-day level-4"></span>

                    <span>"More"</span>
                </div>
            </header>

            <div class="heatmap-scroll">
                <div class="heatmap-content">
                    <WeekdayLabels/>
                    <ActivityHeatmap/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn WeekdayLabels() -> impl IntoView {
    view! {
        <div class="weekday-labels" aria-hidden="true">
            <span>"Mon"</span>
            <span></span>
            <span>"Wed"</span>
            <span></span>
            <span>"Fri"</span>
            <span></span>
            <span>"Sun"</span>
        </div>
    }
}

#[component]
fn ActivityHeatmap() -> impl IntoView {
    // get all logs: 365 before today date, today date, 5 days after
    let today = Local::now().date_naive();
    let from = today - Duration::days(365);
    let to = today + Duration::days(5);

    view! {
        <div class="heatmap">
            {
                (0..371).map(|_| {
                    view! {
                        <div class="heatmap-day level-0"/>
                    }
                }).collect_view()
            }
        </div>
    }
}