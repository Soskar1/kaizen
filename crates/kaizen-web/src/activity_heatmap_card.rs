use chrono::{Duration};
use leptos::{prelude::*};

use crate::ActivityLogRange;

#[component]
pub fn ActivityHeatmapCard(
    logged_activites_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
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
                    <ActivityHeatmap logged_activites_by_day=logged_activites_by_day/>
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
fn ActivityHeatmap(
    logged_activites_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    view! {
        <div class="heatmap">
            {
                move || {
                    logged_activites_by_day.get().map(|logged_activities| {
                        let amount_of_days = (logged_activities.to() - logged_activities.from()).num_days() + 1;

                        (0..amount_of_days).map(|day| {
                            let day = logged_activities.from + Duration::days(day);

                            match logged_activities.activity_duration(day) {
                                Some(_) => {
                                    view! {
                                        <div class="heatmap-day level-1"/>
                                    }
                                }
                                None => {
                                    view! {
                                        <div class="heatmap-day level-0"/>
                                    }
                                }
                            }
                        }).collect_view()
                    })
                }
            }
        </div>
    }
}