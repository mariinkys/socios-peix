use leptos::prelude::*;

use crate::{
    components::page_loading::PageLoadingComponent, core::api::interests::get_all_interests,
};

#[component]
pub fn InterestsList() -> impl IntoView {
    let interests = Resource::new(|| (), |_| get_all_interests());

    view! {
        <Transition fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Todos los Intereses"</h2>
                            <button class="btn btn-warning"
                                on:click=move |_| interests.refetch()
                            >
                                "Recargar"
                            </button>
                            <a class="btn btn-primary" href="/interests/new">"Añadir Interés"</a>
                        </div>
                        <Show
                            when=move || { interests.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                            fallback=|| view! { <p class="text-center">"No hay intereses..."</p> }
                        >
                            { move || {
                                interests.get().map(move |x| {
                                    x.map(move |res| {
                                        let interests_result = RwSignal::new(res);
                                        view! {
                                            <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200">
                                                <table class="table">
                                                    <thead>
                                                    <tr>
                                                        <th></th>
                                                        <th>"Nombre"</th>
                                                        <th>"Descripción"</th>
                                                        <th class="text-right">"Editar"</th>
                                                    </tr>
                                                    </thead>
                                                    <For each=move || interests_result.get() key=|i| i.id children=move |i| {
                                                        view! {
                                                            <tr>
                                                                <th>{i.id.unwrap_or_default()}</th>
                                                                <td>{i.name}</td>
                                                                <td>{i.description}</td>
                                                                <td class="text-right"><a class="btn btn-primary" href=format!("/interests/{}", i.id.unwrap_or_default())>"Ver"</a></td>
                                                            </tr>
                                                        }
                                                    }/>
                                                </table>
                                            </div>
                                        }
                                    })
                                })
                            }}
                        </Show>
                    </div>
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
