mod domain;
mod application;
mod presentation;
mod integration;

use chrono::{Datelike, Duration, Local, NaiveDate};
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::application::activity::{collect_logs, try_log_activity_time};
use crate::application::timer::Timer;
use crate::domain::activity_log_range::ActivityLogRange;
use crate::integration::client::get_activities;
use crate::presentation::activities_tab::ActivitiesTab;
use crate::presentation::activity_heatmap_card::ActivityHeatmapCard;
use crate::presentation::card::Card;
use crate::presentation::new_activity_tab::NewActivityCard;
use crate::presentation::statistics::Statistics;
use crate::presentation::timer_card::TimerCard;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (activities, set_activities) = signal(Vec::<String>::new());
    let (selected_activity, set_selected_activity) = signal(String::new());
    let (logged_activites_by_day, set_logged_activites_by_day) = signal(None::<ActivityLogRange>);
    let (current_card, set_current_card) = signal(Card::Timer);
    let on_activity_add = on_activity_add(set_current_card);
    let on_activity_change = on_activity_change(set_current_card, set_selected_activity, set_logged_activites_by_day);

    let (timer, set_timer) = signal(Timer::new());

    let start_timer = Callback::new(move |()| {
        set_timer.update(|timer| timer.start());
    });

    let pause_timer = Callback::new(move |()| {
        set_timer.update(|timer| timer.pause());
    });

    let stop_timer = stop_timer(timer, set_timer, selected_activity, set_logged_activites_by_day);

    spawn_local(async move {
            match get_activities().await {
            Ok(activities) => { set_activities.set(activities); }
            Err(error) => {leptos::logging::error!("Failed to get activities: {}", error); }
        }
    });

    view! {
        <main class="page">
            <header class="header">
                <h1>"One small thing, every day"</h1>
                <div class="streak">"🔥 41 day streak"</div>
            </header>

            <ActivitiesTab 
                activities=activities
                selected_activity=selected_activity
                on_activity_add=on_activity_add
                on_activity_change=on_activity_change
                timer=timer
            />

            <section class="dashboard">
                { 
                    move || match current_card.get() {
                        Card::Timer => {
                            view! {
                                <TimerCard
                                    selected_activity=selected_activity
                                    timer=timer
                                    start_timer=start_timer
                                    pause_timer=pause_timer
                                    stop_timer=stop_timer
                                />
                            }
                            .into_any()
                        },
                        Card::NewActivity => {
                            view! {
                                <NewActivityCard set_activities=set_activities/>
                            }
                            .into_any()
                        }
                    }
                }

                <Statistics logged_activities_by_day=logged_activites_by_day/>
            </section>

            <ActivityHeatmapCard logged_activites_by_day=logged_activites_by_day/>
        </main>
    }
}

fn on_activity_change(
    set_current_card: WriteSignal<Card>,
    set_selected_activity: WriteSignal<String>,
    set_logged_activites_by_day: WriteSignal<Option<ActivityLogRange>>
) -> Callback<String> {
    Callback::new(move |activity_name: String| {
        set_selected_activity.set(activity_name.clone());
        set_current_card.set(Card::Timer);
        refresh_logs(activity_name, set_logged_activites_by_day);
    })
}

fn refresh_logs(
    activity_name: String,
    set_logged_activites_by_day: WriteSignal<Option<ActivityLogRange>>
) {
    let current_week_start = get_current_week_start();
    let from = current_week_start - Duration::weeks(52);
    let to = from + Duration::days(370);

    spawn_local(async move {
        match collect_logs(&activity_name, from, to).await {
            Some(logs) => set_logged_activites_by_day.set(Some(logs)),
            None => {
                leptos::logging::error!("No logs received in range from {} to {}.", from, to);
            }
        }
    });
}

pub fn get_current_week_start() -> NaiveDate {
    let today = Local::now().date_naive();
    today - Duration::days(today.weekday().num_days_from_monday() as i64)
}

fn on_activity_add(
    set_current_card: WriteSignal<Card>
) -> Callback<()> {
    Callback::new(move |()| {
        set_current_card.set(Card::NewActivity);
    })
}

fn stop_timer(
    timer: ReadSignal<Timer>,
    set_timer: WriteSignal<Timer>,
    selected_activity: ReadSignal<String>,
    set_logged_activites_by_day: WriteSignal<Option<ActivityLogRange>>
) -> Callback<()> {
    Callback::new(move |()| {
        let elapsed_seconds = timer.with_untracked(|timer| timer.elapsed_seconds());
        let activity_name = selected_activity.get_untracked();

        set_timer.update(|timer| timer.stop());

        let today = Local::now().date_naive();

        spawn_local(async move {
            if try_log_activity_time(&activity_name, elapsed_seconds, today)
            .await
            .is_err() {
                leptos::logging::error!("Failed to log time!");
                return;
            }

            refresh_logs(activity_name, set_logged_activites_by_day);
        });
    })
}