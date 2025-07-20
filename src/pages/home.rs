use leptos::prelude::*;

use crate::components::members::today_members::TodayMembers;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <TodayMembers/>
    }
}
