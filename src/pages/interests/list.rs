use leptos::prelude::*;

use crate::components::interests::list::InterestsList;

#[component]
pub fn InterestsPage() -> impl IntoView {
    view! {
        <InterestsList/>
    }
}
