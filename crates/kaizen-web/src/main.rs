mod client;
mod timer_card;
mod activities_tab;
mod card;

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::client::get_activities;
use crate::timer_card::TimerCard;
use crate::activities_tab::ActivitiesTab;
use crate::card::Card;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (activities, set_activities) = signal(Vec::<String>::new());
    
    let selected_activity_signal = signal(String::new());
    let (selected_activity, _) = selected_activity_signal;

    let (current_card, set_current_card) = signal(Card::Timer);

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

            <ActivitiesTab activities={activities} selected_activity_signal={selected_activity_signal} set_current_card={set_current_card}/>

            <section class="dashboard">
                { 
                    move || match current_card.get() {
                        Card::Timer => {
                            view! {
                                <TimerCard selected_activity={selected_activity}/>
                            }
                            .into_any()
                        },
                        Card::NewActivity => {
                            view! {
                                <NewActivityCard />
                            }
                            .into_any()
                        }
                    }
                }

                <aside class="statistics">
                    <StatCard title="TODAY" value="45m"/>
                    <StatCard title="THIS WEEK" value="5h 20m"/>
                    <StatCard title="BEST DAY" value="3h 10m"/>
                </aside>
            </section>
        </main>
    }
}

#[component]
fn NewActivityCard() -> impl IntoView {
    view! {
        <article class="timer-card">
            <span class="label">"New Activity"</span>
        </article>
    }
}

#[component]
fn StatCard(
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