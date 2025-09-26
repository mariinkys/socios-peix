// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

use crate::{components::page_loading::PageLoadingComponent, core::api::users::get_all_users};

#[component]
pub fn UsersList() -> impl IntoView {
    let users = OnceResource::new(get_all_users());

    view! {
        <Transition fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex flex-col md:flex-row gap-3 md:gap-1">
                            <h2 class="card-title grow">"Todos los Usuarios"</h2>

                            <div class="flex flex-col md:flex-row gap-2 w-full md:w-auto">
                                <a class="btn btn-primary" href="/users/new">"Añadir Usuario"</a>
                            </div>
                        </div>

                        <Show
                            when=move || { users.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                            fallback=|| view! { <p class="text-center mt-3">"No hay usuarios..."</p> }
                        >
                            <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200">
                                <table class="table">
                                    <thead>
                                    <tr>
                                        <th></th>
                                        <th>"Nombre de Usuario"</th>
                                        <th>"Creado"</th>
                                        <th>"Actualizado"</th>
                                        <th class="text-right">"Editar"</th>
                                    </tr>
                                    </thead>
                                    <tbody>
                                        <For each=move || users.get_untracked().and_then(|res| res.ok()).unwrap_or_default() key=|u| u.id children=move |u| {
                                            view! {
                                                <tr>
                                                    <th>{u.id.unwrap_or_default()}</th>
                                                    <td>{u.username}</td>
                                                    <td>{u.created_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                    <td>{u.updated_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                    <td class="text-right"><a class="btn btn-primary" href=format!("/users/{}", u.id.unwrap_or_default())>"Ver"</a></td>
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
