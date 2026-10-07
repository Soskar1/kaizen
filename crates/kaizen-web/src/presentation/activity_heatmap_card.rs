use chrono::{Duration};
use leptos::{prelude::*};

use crate::{ActivityLogRange, presentation::text_formattings::format_hours_minutes};

#[component]
pub fn ActivityHeatmapCard(
    logged_activites_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    view! {
        <section class="heatmap-card">
            <header class="heatmap-card-header">
                <TotalLoggedTimeLabel logged_activites_by_day=logged_activites_by_day/>

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
fn TotalLoggedTimeLabel(
    logged_activites_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    view! {
        <div class="heatmap-total">
        {
            move || {
                logged_activites_by_day.get().map(|logged_activities| {
                    let duration = logged_activities.duration_sum();
                    let time = format_hours_minutes(duration);

                    view! {
                        <strong>{time}</strong>
                        <span>" logged in the last year"</span>
                    }
                })
            }
        }
        </div>
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

                        (0..amount_of_days).map(|day_offset| {
                            let day = logged_activities.from() + Duration::days(day_offset);
                            let duration = logged_activities.activity_duration(day).unwrap_or_default();
                            let class = heatmap_day_class(duration);
                            
                            let duration_text = format_hours_minutes(duration);
                            let tooltip_text = format!("{day}\n{duration_text}");

                            view! {
                                <button
                                    type="button"
                                    class=class
                                    data-tooltip=tooltip_text.clone()
                                    aria-label=tooltip_text
                                />
                            }
                        }).collect_view()
                    })
                }
            }
        </div>
    }
}

fn heatmap_day_class(duration_seconds: u64) -> &'static str {
    const HOUR: u64 = 60 * 60;

    match duration_seconds {
        0 => "heatmap-day level-0",
        seconds if seconds < HOUR => "heatmap-day level-1",
        seconds if seconds < 2 * HOUR => "heatmap-day level-2",
        seconds if seconds < 3 * HOUR => "heatmap-day level-3",
        _ => "heatmap-day level-4"
    }
}