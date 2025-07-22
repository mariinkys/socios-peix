use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    components::{interests::upsert::UpsertInterest, page_loading::PageLoadingComponent},
    core::{api::interests::get_interest, models::interest::Interest},
};

#[derive(Params, PartialEq)]
struct InterestParams {
    id: Option<i32>,
}

#[component]
pub fn UpsertInterestPage() -> impl IntoView {
    let params = use_params::<InterestParams>();

    let interest_model = RwSignal::new(Interest::default());
    let edit_mode = RwSignal::new(false);

    let interest_resource = Resource::new(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        move |params| async move {
            match params {
                Some(id) => get_interest(id).await,
                None => {
                    edit_mode.set(true);
                    Ok(Interest::default())
                }
            }
        },
    );

    Effect::new(move |_| {
        if let Some(resource_result) = interest_resource.get() {
            match resource_result {
                Ok(interest) => interest_model.set(interest),
                Err(_) => interest_model.set(Interest::default()),
            }
        }
    });

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="flex flex-row gap-2 mb-3 items-center">
                    <h2 class="text-2xl grow">"Detalles Interés"</h2>
                    <button
                        class="btn"
                        class:btn-accent=move || !edit_mode.get()
                        class:btn-error=move || edit_mode.get()
                        disabled=move || interest_model.get().id.is_none()
                        on:click=move |_| edit_mode.update(|val| *val = !*val)
                    >
                        "Editar"
                    </button>
                </div>

                <UpsertInterest edit_mode=edit_mode model=interest_model/>
            </ErrorBoundary>
        </Suspense>
    }
}
