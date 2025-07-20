use leptos::prelude::*;

use crate::{
    components::page_loading::PageLoadingComponent, core::api::members::get_today_members,
};

#[component]
pub fn TodayMembers() -> impl IntoView {
    let today_members = Resource::new(|| (), |_| get_today_members());

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Miembros destacados hoy"</h2>
                            <button class="btn btn-warning"
                                on:click=move |_| today_members.refetch()
                            >
                                "Recargar"
                            </button>
                            <button class="btn btn-primary">"Añadir Miembro"</button>
                        </div>
                        <Show
                            when=move || { today_members.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                            fallback=|| view! { <p class="text-center">"No hay miembros..."</p> }
                        >
                            <p>"Body Wehehe"</p>
                        </Show>

                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
