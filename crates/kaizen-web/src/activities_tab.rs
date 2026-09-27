use leptos::prelude::*;
use crate::card::Card;

#[component]
pub fn ActivitiesTab(
    activities: ReadSignal<Vec<String>>,
    selected_activity_signal: (ReadSignal<String>, WriteSignal<String>),
    set_current_card: WriteSignal<Card>
) -> impl IntoView {
    view! {
        <nav class="activities">
            <For
                each=move || activities.get()
                key=|name| name.clone()
                children=move |name| {
                    view! {
                        <ActivityButton button_content={name} selected_activity_signal={selected_activity_signal} set_current_card={set_current_card}/>
                    }
                }
            />

            <button 
                type="button"
                class="add-activity-button" on:click=move |_| {
                    set_current_card.set(Card::NewActivity);
            }/>
        </nav>
    }
}

#[component]
fn ActivityButton(
    button_content: String,
    selected_activity_signal: (ReadSignal<String>, WriteSignal<String>),
    set_current_card: WriteSignal<Card>
) -> impl IntoView {
    let name_for_class = button_content.clone();
    let name_for_click = button_content.clone();

    let (selected_activity, set_selected_activity) = selected_activity_signal;

    view! {
        <button class:active=move || {
            selected_activity.with(|selected| {
                selected == &name_for_class
            })
        }
        on:click=move |_| {
            set_selected_activity.set(name_for_click.clone());
            set_current_card.set(Card::Timer);
        }>
            {button_content}
        </button>
    }
}