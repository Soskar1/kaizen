use leptos::prelude::*;

#[component]
pub fn StreakCard() -> impl IntoView {
    view! {
        <article class="streak-card">
            <div class="streak-summary">
                <svg class="streak-flame" viewBox="0 0 32 36" aria-hidden="true">
                    <path
                        class="streak-flame-outer"
                        d="M16 1C18 8 29 13 29 23C29 30.5 23.2 35 16 35C8.8 35 3 30.5 3 23.5C3 17.5 6.5 13.5 10 11C10 15.5 11.5 18.5 14 19.5C12.5 13 13 6.5 16 1Z"
                    />
                    <path
                        class="streak-flame-inner"
                        d="M16 20.5C20 23.5 22 26 22 29C22 32 19.3 34 16 34C12.7 34 10 32 10 29C10 26 12 23.5 16 20.5Z"
                    />
                </svg>

                <strong class="streak-count">"41"</strong>

                <div class="streak-title">
                    <span class="streak-title-main">"day streak"</span>
                    <span class="streak-title-sub">"any activity · best 112"</span>
                </div>
            </div>

            <ol class="streak-week" aria-label="This week">
                <StreakDay label="M" done=true/>
                <StreakDay label="T" done=true/>
                <StreakDay label="W" done=true/>
                <StreakDay label="T" done=true/>
                <StreakDay label="F" done=true/>
                <StreakDay label="S" done=true/>
                <StreakDay label="S" today=true/>
            </ol>

            <div class="streak-goal">
                <div class="streak-goal-header">
                    <span class="streak-goal-title">"9 days to 50"</span>
                    <span class="streak-goal-count">"41 / 50"</span>
                </div>

                <div
                    class="streak-progress"
                    role="progressbar"
                    aria-valuemin="0"
                    aria-valuemax="50"
                    aria-valuenow="41"
                >
                    <div class="streak-progress-fill" style="width: 82%"></div>
                </div>

                <span class="streak-warning">"Nothing logged today · 4h 20m left"</span>
            </div>
        </article>
    }
}

#[component]
fn StreakDay(
    #[prop(into)] label: String,
    #[prop(optional)] done: bool,
    #[prop(optional)] today: bool
) -> impl IntoView {
    view! {
        <li class="streak-day" class:done=done class:today=today>
            <span class="streak-day-circle">
                {done.then(|| view! {
                    <svg viewBox="0 0 16 16" aria-hidden="true">
                        <path d="M3.5 8.5L6.5 11.5L12.5 5" />
                    </svg>
                })}
            </span>
            <span class="streak-day-label">{label}</span>
        </li>
    }
}
