use std::time::{Duration};

use leptos::prelude::*;

// Need to use web_time instead of std, because Instant from std panics in web app
// https://crates.io/crates/web-time
use web_time::Instant;

#[derive(Clone)]
enum TimerControl {
    Play,
    PauseStop,
    ResumeStop
}

#[component]
pub fn TimerCard(
    selected_activity: ReadSignal<String>
) -> impl IntoView {
    let (elapsed_seconds, set_elapsed_seconds) = signal(0_u64);
    let (accumulated_time_in_seconds, set_accumulated_time_in_seconds) = signal(0_64);
    let (started_at, set_started_at) = signal(None::<Instant>);
    let (timer_control_button, set_timer_control_button) = signal(TimerControl::Play);

    let interval_handle = set_interval_with_handle(
        move || {
            if let Some(started_at) = started_at.get_untracked() {
                set_elapsed_seconds.set(started_at.elapsed().as_secs());
            }
        },
        Duration::from_millis(200))
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
                {
                    move || {
                        let total_seconds = elapsed_seconds.get() + accumulated_time_in_seconds.get();
                        format_timer(total_seconds)
                    }
                }
            </div>

            {
                move || match timer_control_button.get() {
                    TimerControl::Play => {
                        view! {
                            <StartTimer
                                set_started_at=set_started_at
                                set_timer_control_button=set_timer_control_button
                            />
                        }
                        .into_any()
                    },
                    TimerControl::PauseStop => {
                        view! {
                            <PauseStopTimer
                                elapsed_seconds=elapsed_seconds
                                set_elapsed_seconds=set_elapsed_seconds
                                set_accumulated_time_in_seconds=set_accumulated_time_in_seconds
                                set_started_at=set_started_at
                                set_timer_control_button=set_timer_control_button
                            />
                        }
                        .into_any()
                    },
                    TimerControl::ResumeStop => {
                        view! {
                            <ResumeStopTimer
                                set_elapsed_seconds=set_elapsed_seconds
                                set_accumulated_time_in_seconds=set_accumulated_time_in_seconds
                                set_started_at=set_started_at
                                set_timer_control_button=set_timer_control_button
                            />
                        }
                        .into_any()
                    }
                }
            }
        </article>
    }
}



#[component]
fn StartTimer(
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) -> impl IntoView {
    view! {
        <button
            class="start-button"
            on:click=move |_| start_timer(set_started_at, set_timer_control_button)
        >
            "▶ Start"
        </button>
    }
}

#[component]
fn PauseStopTimer(
    elapsed_seconds: ReadSignal<u64>,
    set_elapsed_seconds: WriteSignal<u64>,
    set_accumulated_time_in_seconds: WriteSignal<u64>,
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) -> impl IntoView {
    view! {
        <div class="timer-control-buttons">
            <button
                type="button"
                class="timer-control-button pause-button"
                on:click=move |_| {
                    set_started_at.set(None);
                    set_timer_control_button.set(TimerControl::ResumeStop);

                    set_accumulated_time_in_seconds.update(move |current_time| {
                        *current_time += elapsed_seconds.get()
                    });

                    set_elapsed_seconds.set(0);
                }
            >
                "⏸ Pause"
            </button>

            <StopTimerButton
                set_elapsed_seconds=set_elapsed_seconds
                set_accumulated_time_in_seconds=set_accumulated_time_in_seconds
                set_started_at=set_started_at
                set_timer_control_button=set_timer_control_button
            />
        </div>
    }
}

#[component]
fn ResumeStopTimer(
    set_elapsed_seconds: WriteSignal<u64>,
    set_accumulated_time_in_seconds: WriteSignal<u64>,
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) -> impl IntoView {
    view! {
        <div class="timer-control-buttons">
            <button
                type="button"
                class="timer-control-button resume-button"
                on:click=move |_| {
                    start_timer(
                        set_started_at,
                        set_timer_control_button,
                    );
                }
            >
                <span aria-hidden="true">"▶"</span>
                <span>"Resume"</span>
            </button>

            <StopTimerButton
                set_elapsed_seconds=set_elapsed_seconds
                set_accumulated_time_in_seconds=set_accumulated_time_in_seconds
                set_started_at=set_started_at
                set_timer_control_button=set_timer_control_button
            />
        </div>
    }
}

#[component]
fn StopTimerButton(
    set_elapsed_seconds: WriteSignal<u64>,
    set_accumulated_time_in_seconds: WriteSignal<u64>,
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="timer-control-button stop-button"
            on:click=move |_| {
                set_elapsed_seconds.set(0);
                set_accumulated_time_in_seconds.set(0);
                set_started_at.set(None);
                set_timer_control_button.set(TimerControl::Play);

                // TODO: save to file
            }
        >
            "⏹ Stop"
        </button>
    }
}

fn start_timer(
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) {
    set_started_at.set(Some(Instant::now()));
    set_timer_control_button.set(TimerControl::PauseStop);
}

fn format_timer(duration_in_seconds: u64) -> String {
    let hours = duration_in_seconds / 3600;
    let minutes = (duration_in_seconds % 3600) / 60;
    let seconds = duration_in_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}