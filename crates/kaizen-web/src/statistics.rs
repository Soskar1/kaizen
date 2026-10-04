use leptos::{prelude::*};

#[component]
pub fn Staticstics(
    
) -> impl IntoView {
    view! {
        <aside class="statistics">
            <StatisticCard title="TODAY" value="45m"/>
            <StatisticCard title="THIS WEEK" value="5h 20m"/>
            <StatisticCard title="BEST DAY" value="3h 10m"/>
        </aside>
    }
}

#[component]
fn StatisticCard(
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