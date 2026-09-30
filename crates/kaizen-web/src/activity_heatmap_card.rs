use chrono::NaiveDate;
use leptos::prelude::*;

struct DayActivity {
    duration_in_seconds: u64,
    date: NaiveDate
}

#[component]
pub fn ActivityHeatmapCard() -> impl IntoView {
    view! {
        <section class="heatmap-card">
            <div class="heatmap-scroll">
                <ActivityHeatmap/>
            </div>
        </section>
    }
}

#[component]
fn ActivityHeatmap() -> impl IntoView {
    view! {
        <div class="heatmap">
            {
                (0..371).map(|_| {
                    view! {
                        <div class="heatmap-day"/>
                    }
                }).collect_view()
            }
        </div>
    }
}