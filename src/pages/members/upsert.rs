use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    components::{
        interests::member_interests_upsert::MemberInterestsUpsert, members::upsert::UpsertMember,
        page_loading::PageLoadingComponent,
    },
    core::{api::members::get_member, models::member::Member},
};

#[derive(Params, PartialEq)]
struct MemberParams {
    id: Option<i32>,
}

#[component]
pub fn UpsertMemberPage() -> impl IntoView {
    let params = use_params::<MemberParams>();

    let member_model = RwSignal::new(Member::default());
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

    Effect::new(move |_| {
        if let Some(resource_result) = member_resource.get() {
            match resource_result {
                Ok(member) => member_model.set(member),
                Err(_) => member_model.set(Member::default()),
            }
        }
    });

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="flex flex-row gap-2 mb-3 items-center">
                    <h2 class="text-2xl grow">"Detalles Socio"</h2>
                    <button
                        class="btn"
                        class:btn-accent=move || !edit_mode.get()
                        disabled=move || member_model.get().id.is_none()
                        on:click=move |_| edit_mode.update(|val| *val = !*val)
                    >
                        "Editar"
                    </button>
                </div>

                <div class="flex flex-col md:flex-row w-full gap-2 h-full md:h-[80vh]">
                    <UpsertMember edit_mode=edit_mode model=member_model/>
                    <Show
                        when=move || { member_model.get().id.is_some() }
                        fallback=|| view! { <p></p> }
                    >
                        <MemberInterestsUpsert edit_mode=edit_mode member_id=member_model.get_untracked().id.unwrap()/>
                    </Show>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
