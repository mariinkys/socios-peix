use leptos::prelude::*;

use crate::{components::page_loading::PageLoadingComponent, core::api::members::get_all_members};

#[component]
pub fn MembersList() -> impl IntoView {
    let members = OnceResource::new(get_all_members());

    view! {
        <Transition fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Todos los Socios"</h2>
                            <a class="btn btn-primary" href="/members/new">"Añadir Socio"</a>
                        </div>
                        <Show
                            when=move || { members.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                            fallback=|| view! { <p class="text-center">"No hay socios..."</p> }
                        >
                            <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200">
                                <table class="table">
                                    <thead>
                                    <tr>
                                        <th></th>
                                        <th>"Nombre Completo"</th>
                                        <th>"Email"</th>
                                        <th>"Fecha de Nacimiento"</th>
                                        <th>"Género"</th>
                                        <th>"País"</th>
                                        <th class="text-right">"Editar"</th>
                                    </tr>
                                    </thead>
                                    <tbody>
                                        <For each=move || members.get_untracked().and_then(|res| res.ok()).unwrap_or_default() key=|m| m.id children=move |m| {
                                            view! {
                                                <tr>
                                                    <th>{m.id.unwrap_or_default()}</th>
                                                    <td>{format!("{} {} {}", m.name, m.surname, m.second_surname)}</td>
                                                    <td>{m.email}</td>
                                                    <td>{m.birthdate.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                    <td>{m.gender.to_string()}</td>
                                                    <td>{m.country.to_string()}</td>
                                                    <td class="text-right"><a class="btn btn-primary" href=format!("/members/{}", m.id.unwrap_or_default())>"Ver"</a></td>
                                                </tr>
                                            }
                                        }/>
                                    </tbody>
                                </table>
                            </div>
                        </Show>
                    </div>
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
