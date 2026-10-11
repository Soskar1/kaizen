use leptos::prelude::*;

#[component]
pub fn WelcomeCard(
    on_activity_add: Callback<()>
) -> impl IntoView {
    view! {
        <article class="welcome-card">
            <div class="welcome-content">
                <span class="label">"GET STARTED"</span>

                <h2>"What will you improve today?"</h2>

                <p class="welcome-description">
                    "Pick an activity above to start the timer, or create a new one. Even five minutes counts."
                </p>

                <ol class="welcome-steps">
                    <li>
                        <span class="welcome-step-number">"1"</span>
                        <span>"Pick an activity"</span>
                    </li>
                    <li>
                        <span class="welcome-step-number">"2"</span>
                        <span>"Start the timer"</span>
                    </li>
                    <li>
                        <span class="welcome-step-number">"3"</span>
                        <span>"Keep the streak alive"</span>
                    </li>
                </ol>

                <button
                    type="button"
                    class="create-activity-button welcome-add-button"
                    on:click=move |_| on_activity_add.run(())
                >
                    <span class="create-activity-icon" aria-hidden="true">"+"</span>
                    <span>"New activity"</span>
                </button>
            </div>

            <div class="welcome-illustration" aria-hidden="true">
                <div class="welcome-ring">
                    <span>"00:00"</span>
                </div>
            </div>
        </article>
    }
}
