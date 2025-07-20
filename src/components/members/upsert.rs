use leptos::prelude::*;

use crate::{components::page_loading::PageLoadingComponent, core::models::member::Member};

#[component]
pub fn UpsertMember(edit_mode: RwSignal<bool>, member: Member) -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <p>"Bip bip, upsert component..."</p>
            </ErrorBoundary>
        </Suspense>
    }
}
