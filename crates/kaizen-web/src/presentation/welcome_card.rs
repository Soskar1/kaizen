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
                <div class="welcome-tiles">
                    <span class="welcome-tile level-0"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-0"></span>
                    <span class="welcome-tile level-0"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-0"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-0"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-0"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-0"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-3"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-3"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-3"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-4"></span>
                    <span class="welcome-tile level-3"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-1"></span>
                    <span class="welcome-tile level-3"></span>
                    <span class="welcome-tile level-2"></span>
                    <span class="welcome-tile level-4"></span>
                    <span class="welcome-tile level-3"></span>
                    <span class="welcome-tile level-4"></span>
                    <span class="welcome-tile level-4"></span>
                </div>
            </div>
        </article>
    }
}
