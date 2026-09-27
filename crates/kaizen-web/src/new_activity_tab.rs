use leptos::{prelude::*, reactive::{spawn, spawn_local}};

use crate::client::{ServerError, create_activity};

#[component]
pub fn NewActivityCard() -> impl IntoView {
    let activity_name = RwSignal::new("".to_string());

    view! {
        <article class="new-activity-card">
            <div class="new-activity-content">
                <h2>"Create new activity"</h2>

                <p class="new-activity-description">
                    "Enter something you want to spend a little more time on."
                </p>

                <div class="new-activity-form">
                    <label class="activity-name-field">
                        <span>"ACTIVITY NAME"</span>

                        <input
                            type="text"
                            placeholder="Learning"
                            maxlength="50"
                            autocomplete="off"
                            bind:value=activity_name
                        />
                    </label>

                    <button
                        type="button"
                        class="create-activity-button"
                        on:click=move |_| {
                            let activity_name = activity_name.get();

                            spawn_local(async move {
                                let result = create_activity(&activity_name).await;

                                match result {
                                    Ok(()) => {},
                                    Err(error) => {leptos::logging::error!("Failed to create an activity: {}", error); }
                                }
                            });
                        }
                    >
                        <span
                            class="create-activity-icon"
                            aria-hidden="true"
                        >
                            "+"
                        </span>

                        <span>"Create"</span>
                    </button>
                </div>
            </div>
        </article>
    }
}