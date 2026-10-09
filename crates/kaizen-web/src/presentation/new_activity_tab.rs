use leptos::{prelude::*, reactive::{spawn_local}};

use crate::application::activity::try_create_activity;

#[component]
pub fn NewActivityCard(
    set_activities: WriteSignal<Vec<String>>
) -> impl IntoView {
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
                                if let Err(_) = try_create_activity(&activity_name).await {
                                    leptos::logging::error!("Failed to create an activity.");
                                } else {
                                    set_activities.update(|activities| {
                                        activities.push(activity_name);
                                        activities.sort_by_key(|name| name.to_lowercase());
                                    });
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