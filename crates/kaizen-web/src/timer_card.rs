use std::time::{Duration};

use leptos::prelude::*;
use web_time::Instant;

#[component]
pub fn TimerCard(
    selected_activity: ReadSignal<String>
) -> impl IntoView {
    let elapsed_seconds = RwSignal::new(0_u64);
    let started_at = RwSignal::new(None::<Instant>);

    let interval_handle = set_interval_with_handle(
        move || {
            if let Some(started_at) = started_at.get_untracked() {
                elapsed_seconds.set(started_at.elapsed().as_secs());
            }
        },
        Duration::from_secs(1))
        .expect("Failed to create a timer interval");

    on_cleanup(move || {
        interval_handle.clear();
    });

    view! {
        <article class="timer-card">
            <span class="label">"TIMER"</span>
            <span class="category">
                {move || selected_activity.get()}
            </span>

            <div class="timer">
                {move || format_timer(elapsed_seconds.get())}
            </div>

            <button
                class="start-button"
                disabled=move || started_at.get().is_some()
                on:click=move |_| {
                    started_at.set(Some(Instant::now()));
                }
            >
                "▶ Start"
            </button>
        </article>
    }
}

fn format_timer(duration_in_seconds: u64) -> String {
    let hours = duration_in_seconds / 3600;
    let minutes = (duration_in_seconds % 3600) / 60;
    let seconds = duration_in_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}