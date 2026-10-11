use chrono::{Datelike, Duration};
use leptos::{prelude::*};

use crate::{ActivityLogRange, presentation::text_formattings::format_hours_minutes};

#[component]
pub fn ActivityHeatmapCard(
    logged_activites_by_day: ReadSignal<Option<ActivityLogRange>>
) -> impl IntoView {
    let month_labels = Signal::derive(move || {
        logged_activites_by_day.with(|logs| {
            match logs {
                Some(logs) => {
                    const DAYS_IN_COLUMN: i64 = 7;

                    let amount_of_columns = logs.total_days() / DAYS_IN_COLUMN;
                    let column_start = |column: i64| logs.from() + Duration::days(column * DAYS_IN_COLUMN);

                    (0..amount_of_columns)
                        .filter(|&column| {
                            column == 0 || column_start(column).month() != column_start(column - 1).month()
                        })
                        .map(|column| (column_start(column).format("%b").to_string(), column))
                        .collect()
                }
                None => vec![]
            }
        })
    });

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

                    <div class="heatmap-grid">
                        <MonthLabels labels=month_labels/>
                        <ActivityHeatmap logged_activites_by_day=logged_activites_by_day/>
                    </div>
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
fn MonthLabels(
    #[prop(into)] labels: Signal<Vec<(String, i64)>>
) -> impl IntoView {
    view! {
        <div class="month-labels" aria-hidden="true">
            <For
                each=move || labels.get()
                key=|(_, column)| *column
                children=|(name, column)| {
                    view! {
                        <span style=format!("grid-column: {}", column + 1)>{name}</span>
                    }
                }
            />
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
                        let amount_of_days = logged_activities.total_days();

                        (0..amount_of_days).map(|day_offset| {
                            let day = logged_activities.from() + Duration::days(day_offset);
                            let duration = logged_activities.activity_duration(day).unwrap_or_default();
                            let class = heatmap_day_class(duration);
                            
                            let duration_text = format_hours_minutes(duration);
                            let tooltip_text = format!("{day}\n{duration_text}");

                            // TODO: tooltip here was made with workarounds
                            // If this becomes a problem, maybe we need to redesign it
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