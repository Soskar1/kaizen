use std::time::{Duration};

use leptos::prelude::*;

// Need to use web_time instead of std, because Instant from std panics in web app
// https://crates.io/crates/web-time
use web_time::Instant;

#[derive(Clone)]
enum TimerControlButton {
    Play,
    StopPause
}

#[component]
pub fn TimerCard(
    selected_activity: ReadSignal<String>
) -> impl IntoView {
    let elapsed_seconds = RwSignal::new(0_u64);
    let (started_at, set_started_at) = signal(None::<Instant>);
    let (timer_control_button, set_timer_control_button) = signal(TimerControlButton::Play);

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

            {
                move || match timer_control_button.get() {
                    TimerControlButton::Play => {
                        view! {
                            <StartTimer set_started_at=set_started_at set_timer_control_button=set_timer_control_button/>
                        }
                        .into_any()
                    },
                    TimerControlButton::StopPause => {
                        view! {
                            <StopPauseTimer set_timer_control_button=set_timer_control_button/>
                        }
                        .into_any()
                    }
                }
            }
        </article>
    }
}

fn format_timer(duration_in_seconds: u64) -> String {
    let hours = duration_in_seconds / 3600;
    let minutes = (duration_in_seconds % 3600) / 60;
    let seconds = duration_in_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

#[component]
fn StartTimer(
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControlButton>
) -> impl IntoView {
    view! {
        <button
            class="start-button"
            on:click=move |_| {
                set_started_at.set(Some(Instant::now()));
                set_timer_control_button.set(TimerControlButton::StopPause);
            }
        >
            "▶ Start"
        </button>
    }
}

#[component]
fn StopPauseTimer(
    set_timer_control_button: WriteSignal<TimerControlButton>
) -> impl IntoView {
    view! {
        <div class="timer-control-buttons">
            <button
                type="button"
                class="timer-control-button pause-button"
            >
                "⏸ Pause"
            </button>

            <button
                type="button"
                class="timer-control-button stop-button"
            >
                "⏹ Stop"
            </button>
        </div>
    }
}