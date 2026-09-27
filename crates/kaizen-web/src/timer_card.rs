use leptos::prelude::*;

#[component]
pub fn TimerCard(
    selected_activity: ReadSignal<String>
) -> impl IntoView {
    view! {
        <article class="timer-card">
            <span class="label">"TIMER"</span>
            <span class="category">
                {move || selected_activity.get()}
            </span>

            <div class="timer">"00:00:00"</div>

            <button class="start-button">
                "▶ Start"
            </button>
        </article>
    }
}