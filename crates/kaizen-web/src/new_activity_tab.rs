use leptos::prelude::*;

#[component]
pub fn NewActivityCard() -> impl IntoView {
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
                        />
                    </label>

                    <button
                        type="button"
                        class="create-activity-button"
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