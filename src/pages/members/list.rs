use leptos::prelude::*;

use crate::components::members::list::MembersList;

#[component]
pub fn MembersPage() -> impl IntoView {
    view! {
        <MembersList/>
    }
}
