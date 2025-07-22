use leptos::prelude::*;

use crate::{components::page_loading::PageLoadingComponent, core::api::members::get_all_members};

#[component]
pub fn MembersList() -> impl IntoView {
    let members = OnceResource::new(get_all_members());

    let search_bar = RwSignal::new(String::new());
    let search_query = RwSignal::new(String::new());
    Effect::new(move |_| {
        let search_value = search_bar.get();
        set_timeout(
            move || {
                search_query.set(search_value.clone());
            },
            std::time::Duration::from_millis(300),
        );
    });

    let filtered_members = Memo::new(move |_| {
        if let Some(Ok(members_list)) = members.get() {
            let query = search_query.get().to_lowercase();

            if query.is_empty() {
                Some(members_list)
            } else {
                Some(
                    members_list
                        .into_iter()
                        .filter(|member| {
                            let full_name = format!(
                                "{} {} {}",
                                member.name, member.surname, member.second_surname
                            )
                            .to_lowercase();

                            full_name.contains(&query)
                                || member.email.to_lowercase().contains(&query)
                                || member.country.to_string().to_lowercase().contains(&query)
                                || member.phone.to_string().to_lowercase().contains(&query)
                        })
                        .collect(),
                )
            }
        } else {
            None
        }
    });

    view! {
        <Transition fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex justify-between gap-1">
                            <h2 class="card-title">"Todos los Socios"</h2>
                            <input class="input input-primary" type="text"
                                on:input:target=move |ev| {
                                    search_bar.set(ev.target().value());
                                }
                                prop:value=search_bar
                            />
                            <a class="btn btn-primary" href="/members/new">"Añadir Socio"</a>
                        </div>

                        <Suspense fallback=|| view! { <p class="text-center">"Cargando socios..."</p> }>
                            { move || {
                                filtered_members.get().map(|members_list|  {
                                    if members_list.is_empty() {
                                        view! { <p class="text-center mt-3">"No hay socios..."</p> }.into_any()
                                    } else {
                                        view! {
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
                                                        <For each=move || filtered_members.get().unwrap_or_default() key=|m| m.id children=move |m| {
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
                                        }.into_any()
                                    }
                                })
                            }}
                        </Suspense>
                    </div>
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
