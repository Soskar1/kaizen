use chrono::{Local};
use std::time::{Duration};
use leptos::{prelude::*, reactive::spawn_local};

// Need to use web_time instead of std, because Instant from std panics in web app
// https://crates.io/crates/web-time
use web_time::Instant;

use crate::client::{log_activity_time};

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
    let (accumulated_time_in_seconds, set_accumulated_time_in_seconds) = signal(0_u64);
    let (started_at, set_started_at) = signal(None::<Instant>);
    let (timer_control_button, set_timer_control_button) = signal(TimerControl::Play);

    let start_timer = start_timer_callback(set_started_at, set_timer_control_button);

    let pause_timer = pause_timer_callback(
        elapsed_seconds,
        set_elapsed_seconds,
        set_accumulated_time_in_seconds,
        set_started_at,
        set_timer_control_button
    );

    let stop_timer = stop_timer_callback(
        elapsed_seconds,
        accumulated_time_in_seconds,
        selected_activity,
        set_elapsed_seconds,
        set_accumulated_time_in_seconds,
        set_started_at,
        set_timer_control_button
    );

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
                            <StartTimer start_timer=start_timer/>
                        }
                        .into_any()
                    },
                    TimerControl::PauseStop => {
                        view! {
                            <PauseStopTimer
                                pause_timer=pause_timer
                                stop_timer=stop_timer
                            />
                        }
                        .into_any()
                    },
                    TimerControl::ResumeStop => {
                        view! {
                            <ResumeStopTimer
                                start_timer=start_timer
                                stop_timer=stop_timer
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
    start_timer: Callback<()>
) -> impl IntoView {
    view! {
        <button
            class="start-button"
            on:click=move |_| start_timer.run(())
        >
            "▶ Start"
        </button>
    }
}

#[component]
fn PauseStopTimer(
    pause_timer: Callback<()>,
    stop_timer: Callback<()>
) -> impl IntoView {
    view! {
        <div class="timer-control-buttons">
            <button
                type="button"
                class="timer-control-button pause-button"
                on:click=move |_| pause_timer.run(())
            >
                "⏸ Pause"
            </button>

            <StopTimerButton stop_timer=stop_timer/>
        </div>
    }
}

#[component]
fn ResumeStopTimer(
    start_timer: Callback<()>,
    stop_timer: Callback<()>
) -> impl IntoView {
    view! {
        <div class="timer-control-buttons">
            <button
                type="button"
                class="timer-control-button resume-button"
                on:click=move |_| start_timer.run(())
            >
                <span aria-hidden="true">"▶"</span>
                <span>"Resume"</span>
            </button>

            <StopTimerButton stop_timer=stop_timer/>
        </div>
    }
}

#[component]
fn StopTimerButton(
    stop_timer: Callback<()>
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="timer-control-button stop-button"
            on:click=move |_| stop_timer.run(())
        >
            "⏹ Stop"
        </button>
    }
}

fn format_timer(duration_in_seconds: u64) -> String {
    let hours = duration_in_seconds / 3600;
    let minutes = (duration_in_seconds % 3600) / 60;
    let seconds = duration_in_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn start_timer_callback(
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) -> Callback<()> {
    Callback::new(move |()| {
        set_started_at.set(Some(Instant::now()));
        set_timer_control_button.set(TimerControl::PauseStop);
    })
}

fn pause_timer_callback(
    elapsed_seconds: ReadSignal<u64>,
    set_elapsed_seconds: WriteSignal<u64>,
    set_accumulated_time_in_seconds: WriteSignal<u64>,
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) -> Callback<()> {
    Callback::new(move |()| {
        set_started_at.set(None);
        set_timer_control_button.set(TimerControl::ResumeStop);

        set_accumulated_time_in_seconds.update(move |current_time| {
            *current_time += elapsed_seconds.get()
        });

        set_elapsed_seconds.set(0);
    })
}

fn stop_timer_callback(
    elapsed_seconds: ReadSignal<u64>,
    accumulated_time_in_seconds: ReadSignal<u64>,
    selected_activity: ReadSignal<String>,
    set_elapsed_seconds: WriteSignal<u64>,
    set_accumulated_time_in_seconds: WriteSignal<u64>,
    set_started_at: WriteSignal<Option<Instant>>,
    set_timer_control_button: WriteSignal<TimerControl>
) -> Callback<()> {
    Callback::new(move|()| {
        let total_duration = elapsed_seconds.get_untracked() + accumulated_time_in_seconds.get_untracked();

        let activity_name = selected_activity.get_untracked();
        let today = Local::now().date_naive();

        set_elapsed_seconds.set(0);
        set_accumulated_time_in_seconds.set(0);
        set_started_at.set(None);
        set_timer_control_button.set(TimerControl::Play);

        spawn_local(async move {
            if let Err(error) = log_activity_time(&activity_name, total_duration, today).await {
                leptos::logging::error!("Failed to log time! {}", error);
            }
        });
    })
}