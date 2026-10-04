mod client;
mod timer_card;
mod activities_tab;
mod card;
mod new_activity_tab;
mod activity_heatmap_card;
mod statistics;

use std::collections::HashMap;

use chrono::{Datelike, Duration, Local, NaiveDate};
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::client::{get_activities, get_activity_logs};
use crate::timer_card::TimerCard;
use crate::activities_tab::ActivitiesTab;
use crate::card::Card;
use crate::new_activity_tab::NewActivityCard;
use crate::activity_heatmap_card::ActivityHeatmapCard;
use crate::statistics::Staticstics;

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
            />

            <section class="dashboard">
                { 
                    move || match current_card.get() {
                        Card::Timer => {
                            view! {
                                <TimerCard selected_activity=selected_activity/>
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

                <Staticstics logged_activites_by_day=logged_activites_by_day/>
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

        let today = Local::now().date_naive();
        let current_week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
        let from = current_week_start - Duration::weeks(52);
        let to = from + Duration::days(370);

        spawn_local(async move {
            match get_activity_logs(&activity_name, from, to).await {
                Ok(logs) => {
                    let activity_log_range = ActivityLogRange {
                        from,
                        to,
                        logs
                    };
                    set_logged_activites_by_day.set(Some(activity_log_range));
                }
                Err(error) => {
                    leptos::logging::error!("Failed to get activity logs in range from {} to {}. {}", from, to, error);
                }
            }
        });
    })
}

#[derive(Clone)]
pub struct ActivityLogRange {
    from: NaiveDate,
    to: NaiveDate,
    logs: HashMap<NaiveDate, u64>
}

impl ActivityLogRange {
    pub fn from(&self) -> NaiveDate {
        self.from
    }

    pub fn to(&self) -> NaiveDate {
        self.to
    }

    pub fn activity_duration(&self, day: NaiveDate) -> Option<u64> {
        self.logs.get(&day).copied()
    }

    pub fn duration_sum(&self) -> u64 {
        self.logs.values().sum()
    }
}

fn on_activity_add(
    set_current_card: WriteSignal<Card>
) -> Callback<()> {
    Callback::new(move |()| {
        set_current_card.set(Card::NewActivity);
    })
}