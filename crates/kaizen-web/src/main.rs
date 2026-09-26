mod client;

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::client::get_activities;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (activities, set_activities) = signal(Vec::<String>::new());
    
    let selected_activity_signal = signal(String::new());
    let (selected_activity, _) = selected_activity_signal;

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

            <ActivitiesTab activities={activities} selected_activity_signal={selected_activity_signal}/>

            <section class="dashboard">
                <TimerCard selected_activity={selected_activity}/>

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
fn ActivitiesTab(
    activities: ReadSignal<Vec<String>>,
    selected_activity_signal: (ReadSignal<String>, WriteSignal<String>)
) -> impl IntoView {
    view! {
        <nav class="activities">
            <For
                each=move || activities.get()
                key=|name| name.clone()
                children=move |name| {
                    view! {
                        <ActivityButton button_content={name} selected_activity_signal={selected_activity_signal} />
                    }
                }
            />

            <button type="button"
                    class="add-activity-button" on:click=move |_| {
                // TODO: Show add-activity-screen
            }/>
        </nav>
    }
}

#[component]
fn ActivityButton(
    button_content: String,
    selected_activity_signal: (ReadSignal<String>, WriteSignal<String>)
) -> impl IntoView {
    let name_for_class = button_content.clone();
    let name_for_click = button_content.clone();

    let (selected_activity, set_selected_activity) = selected_activity_signal;

    view! {
        <button class:active=move || {
            selected_activity.with(|selected| {
                selected == &name_for_class
            })
        }
        on:click=move |_| {
            set_selected_activity.set(name_for_click.clone());
        }>
            {button_content}
        </button>
    }
}

#[component]
fn TimerCard(
    selected_activity: ReadSignal<String>
) -> impl IntoView {
    view! {
        <article class="timer-card">
            <span class="label">"TIMER"</span>
            <span class="category">
                {move || selected_activity.get()}
            </span>

            <div class="timer">"00:00:00"</div>

            <button class="start-button">
                "▶ Start"
            </button>
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