use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    components::members::upsert::UpsertMember,
    core::{api::members::get_member, models::member::Member},
};

#[derive(Params, PartialEq)]
struct MemberParams {
    id: Option<i32>,
}

#[component]
pub fn UpsertMemberPage() -> impl IntoView {
    let params = use_params::<MemberParams>();

    let edit_mode = RwSignal::new(false);
    let member_resource = Resource::new(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        move |params| async move {
            match params {
                Some(id) => get_member(id).await,
                None => {
                    edit_mode.set(true);
                    Ok(Member::default())
                }
            }
        },
    );

    view! {
        <p>"Bip"</p>
        <UpsertMember edit_mode=edit_mode member=Member::default()/>
    }
}
