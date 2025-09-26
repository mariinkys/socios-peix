use leptos::prelude::*;

use crate::components::users::list::UsersList;

#[component]
pub fn UsersPage() -> impl IntoView {
    view! {
        <UsersList/>
    }
}
