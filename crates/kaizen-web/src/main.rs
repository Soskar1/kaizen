use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (selected_activity, set_selected_activity) = signal("Learning");

    view! {
        <main class="page">
            <header class="header">
                <h1>"One small thing, every day"</h1>
                <div class="streak">"🔥 41 day streak"</div>
            </header>

            <nav class="activities">
                <button
                    class:active=move || selected_activity.get() == "Learning"
                    on:click=move |_| set_selected_activity.set("Learning")>
                    "Learning"
                </button>
                <button
                    class:active=move || selected_activity.get() == "Rust"
                    on:click=move |_| set_selected_activity.set("Rust")>
                    "Rust"
                </button>
                <button
                    class:active=move || selected_activity.get() == "Reading"
                    on:click=move |_| set_selected_activity.set("Reading")>
                    "Reading"
                </button>
            </nav>

            <section class="dashboard">
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

                <aside class="statistics">
                    <StatCard title="TODAY" value="45m"/>
                    <StatCard title="THIS WEEK" value="5h 20m"/>
                    <StatCard title="BEST DAY" value="3h 10m"/>
                </aside>
            </section>
        </main>
    }
}

#[component]
fn StatCard(
    #[prop(into)] title: String,
    #[prop(into)] value: String
) -> impl IntoView {
    view! {
        <article class="stat-card">
            <span class="label">{title}</span>
            <strong>{value}</strong>
        </article>
    }
}