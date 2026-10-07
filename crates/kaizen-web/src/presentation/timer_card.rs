use std::time::{Duration};
use leptos::{prelude::*};

use crate::{application::timer::Timer, domain::timer_state_machine::TimerState, presentation::text_formattings::format_hours_minutes_seconds};

#[component]
pub fn TimerCard(
    selected_activity: ReadSignal<String>,
    timer: ReadSignal<Timer>,
    start_timer: Callback<()>,
    pause_timer: Callback<()>,
    stop_timer: Callback<()>
) -> impl IntoView {
    let (tick, set_tick) = signal(0_u64);

    let interval_handle = set_interval_with_handle(
        move || {
            set_tick.update(|tick| *tick += 1);
        },
        Duration::from_millis(200),
    )
    .expect("Failed to create timer interval");

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
                        tick.get();

                        let elapsed_seconds = timer.with(|timer| {
                            timer.elapsed_seconds()
                        });

                        format_hours_minutes_seconds(elapsed_seconds)
                    }
                }
            </div>

            {
                move || timer.with(|timer| {
                    match timer.state() {
                        TimerState::Idle => {
                            view! {
                                <StartTimer start_timer=start_timer/>
                            }
                            .into_any()
                        },
                        TimerState::Paused { .. } => {
                            view! {
                                <ResumeStopTimer
                                    start_timer=start_timer
                                    stop_timer=stop_timer
                                />
                            }
                            .into_any()
                        },
                        TimerState::Running { .. } => {
                            view! {
                            <PauseStopTimer
                                pause_timer=pause_timer
                                stop_timer=stop_timer
                                />
                            }
                            .into_any()
                        }
                    }
                })
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