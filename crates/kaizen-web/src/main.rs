use std::env;
use std::path::{PathBuf};
use chrono::Utc;
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::logging::{error, log};
use leptos::task::spawn_local;
use kaizen_core::activity::log_activity;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone)]
struct Activity {
    id: u32,
    name: &'static str
}

#[component]
fn App() -> impl IntoView {
    let activities = vec![
        Activity {
            id: 1,
            name: "Learning",
        },
        Activity {
            id: 2,
            name: "Rust",
        },
        Activity {
            id: 3,
            name: "Reading",
        },
    ];



    let (selected_activity, set_selected_activity) = signal(1_u32);

    view! {
        <main class="page">
            <header class="header">
                <h1>"One small thing, every day"</h1>
                <div class="streak">"🔥 41 day streak"</div>
            </header>

            // <nav class="activities">
            //     {activities
            //     .into_iter()
            //     .map(|activity| {
            //         let id = activity.id;
            //         let name = activity.name;
            //
            //         view! {
            //             <button
            //                 class:active=move || selected_activity.get() == id
            //                 on:click=move |_| set_selected_activity.set(id)>
            //                 {name}
            //             </button>
            //         }
            //     })
            //     .collect_view()}
            // </nav>

            <section class="dashboard">
                <article class="timer-card">
                    <span class="label">"TIMER"</span>
                    <span class="category">
                        {move || selected_activity.get()}
                    </span>

                    <div class="timer">"00:00:00"</div>

                    <button class="start-button" on:click=move |_| {
                        spawn_local(async move {
                            let request = Request::post("http://localhost:3000/activities")
                                .header("Content-Type", "text/plain")
                                .body("Learning");

                            let request = match request {
                                Ok(request) => request,
                                Err(error) => {
                                    error!("Failed to build request: {error}");
                                    return;
                                }
                            };
                
                            match request.send().await {
                                Ok(response) if response.ok() => {
                                    log!("Activity directory created");
                                }
                                Ok(response) => {
                                    error!("Server returned status {}", response.status());
                                }
                                Err(error) => {
                                    error!("Request failed: {error}");
                                }
                            }
                        })
                    }>
                        "▶ Start"
                    </button>
                </article>

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