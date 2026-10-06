mod timer_card;
mod activities_tab;
mod new_activity_tab;
mod activity_heatmap_card;
mod statistics;
mod domain;
mod application;
mod presentation;
mod integration;

use chrono::{Datelike, Duration, Local, NaiveDate};
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::application::activity::collect_logs;
use crate::domain::activity_log_range::ActivityLogRange;
use crate::integration::client::get_activities;
use crate::timer_card::TimerCard;
use crate::activities_tab::ActivitiesTab;
use crate::presentation::card::Card;
use crate::new_activity_tab::NewActivityCard;
use crate::activity_heatmap_card::ActivityHeatmapCard;
use crate::statistics::Statistics;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone, PartialEq)]
pub enum TimerState {
    Idle,
    Running
}

#[component]
fn App() -> impl IntoView {
    let (activities, set_activities) = signal(Vec::<String>::new());
    let (selected_activity, set_selected_activity) = signal(String::new());
    let (logged_activites_by_day, set_logged_activites_by_day) = signal(None::<ActivityLogRange>);
    let (timer_state, set_timer_state) = signal(TimerState::Idle);
    let (current_card, set_current_card) = signal(Card::Timer);
    let on_activity_add = on_activity_add(set_current_card);
    let on_activity_change = on_activity_change(set_current_card, set_selected_activity, set_logged_activites_by_day);

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
                timer_state=timer_state
            />

            <section class="dashboard">
                { 
                    move || match current_card.get() {
                        Card::Timer => {
                            view! {
                                <TimerCard
                                    selected_activity=selected_activity
                                    set_timer_state=set_timer_state
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
    })
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