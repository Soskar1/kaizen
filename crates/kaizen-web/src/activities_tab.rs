use leptos::prelude::*;

use crate::TimerState;

#[component]
pub fn ActivitiesTab(
    activities: ReadSignal<Vec<String>>,
    selected_activity: ReadSignal<String>,
    on_activity_add: Callback<()>,
    on_activity_change: Callback<String>,
    timer_state: ReadSignal<TimerState>
) -> impl IntoView {
    view! {
        <nav class="activities">
            <For
                each=move || activities.get()
                key=|name| name.clone()
                children=move |name| {
                    view! {
                        <ActivityButton
                            button_content=name
                            selected_activity=selected_activity
                            on_activity_change=on_activity_change
                            timer_state=timer_state
                        />
                    }
                }
            />

            <button 
                type="button"
                class="add-activity-button" on:click=move |_| on_activity_add.run(())/>
        </nav>
    }
}

#[component]
fn ActivityButton(
    button_content: String,
    selected_activity: ReadSignal<String>,
    on_activity_change: Callback<String>,
    timer_state: ReadSignal<TimerState>
) -> impl IntoView {
    let name_for_class = button_content.clone();
    let name_for_click = button_content.clone();

    view! {
        <button class:active=move || {
            selected_activity.with(|selected| {
                selected == &name_for_class
            })
        }

        disabled=move || timer_state.get() != TimerState::Idle

        on:click=move |_| on_activity_change.run(name_for_click.clone())>
            {button_content}
        </button>
    }
}