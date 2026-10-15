use std::collections::HashMap;

use leptos::prelude::*;

use crate::application::timer::Timer;
use crate::domain::activity_streak::ActivityStreak;

#[component]
pub fn ActivitiesTab(
    activities: ReadSignal<Vec<String>>,
    selected_activity: ReadSignal<String>,
    activity_streaks: ReadSignal<HashMap<String, ActivityStreak>>,
    on_activity_add: Callback<()>,
    on_activity_change: Callback<String>,
    timer: ReadSignal<Timer>
) -> impl IntoView {
    view! {
        <nav class="activities">
            <div class="activity-list">
                <For
                    each=move || activities.get()
                    key=|name| name.clone()
                    children=move |name| {
                        view! {
                            <ActivityButton
                                button_content=name
                                selected_activity=selected_activity
                                activity_streaks=activity_streaks
                                on_activity_change=on_activity_change
                                timer=timer
                            />
                        }
                    }
                />
            </div>

            <button
                type="button"
                class="add-activity-button"
                on:click=move |_| on_activity_add.run(())
                disabled=move || timer.with(|timer| !timer.is_idle())
                />
        </nav>
    }
}

#[component]
fn ActivityButton(
    button_content: String,
    selected_activity: ReadSignal<String>,
    activity_streaks: ReadSignal<HashMap<String, ActivityStreak>>,
    on_activity_change: Callback<String>,
    timer: ReadSignal<Timer>
) -> impl IntoView {
    let name_for_class = button_content.clone();
    let name_for_click = button_content.clone();
    let name_for_streak = button_content.clone();

    let streak = Memo::new(move |_| {
        activity_streaks.with(|streaks| streaks.get(&name_for_streak).copied().unwrap_or_default())
    });

    view! {
        <button class:active=move || {
            selected_activity.with(|selected| {
                selected == &name_for_class
            })
        }

        disabled=move || timer.with(|timer| !timer.is_idle())

        on:click=move |_| on_activity_change.run(name_for_click.clone())>
            <span class="activity-name">{button_content}</span>
            <StreakPill streak=streak/>
        </button>
    }
}

#[component]
fn StreakPill(
    #[prop(into)] streak: Signal<ActivityStreak>
) -> impl IntoView {
    view! {
        <span
            class="activity-streak-pill"
            class:lit=move || streak.with(|streak| streak.current_week_logged())
            aria-label=move || format!("{} week streak", streak.with(|streak| streak.current()))
        >
            <svg class="activity-streak-flame" viewBox="0 0 32 36" aria-hidden="true">
                <path
                    class="activity-streak-flame-outer"
                    d="M16 1C18 8 29 13 29 23C29 30.5 23.2 35 16 35C8.8 35 3 30.5 3 23.5C3 17.5 6.5 13.5 10 11C10 15.5 11.5 18.5 14 19.5C12.5 13 13 6.5 16 1Z"
                />
                <path
                    class="activity-streak-flame-inner"
                    d="M16 20.5C20 23.5 22 26 22 29C22 32 19.3 34 16 34C12.7 34 10 32 10 29C10 26 12 23.5 16 20.5Z"
                />
            </svg>
            <span aria-hidden="true">{move || streak.with(|streak| streak.current())}</span>
        </span>
    }
}
