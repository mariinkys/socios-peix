use leptos::prelude::*;

use crate::{components::page_loading::PageLoadingComponent, core::api::members::get_all_members};

#[component]
pub fn MembersList() -> impl IntoView {
    let members = Resource::new(|| (), |_| get_all_members());

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Todos los Socios"</h2>
                            <button class="btn btn-warning"
                                on:click=move |_| members.refetch()
                            >
                                "Recargar"
                            </button>
                            <a class="btn btn-primary" href="/members/new">"Añadir Socio"</a>
                        </div>
                        <Show
                            when=move || { members.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                            fallback=|| view! { <p class="text-center">"No hay socios..."</p> }
                        >
                            <p>"Body Wehehe"</p>
                        </Show>

                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
