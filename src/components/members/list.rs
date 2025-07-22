use leptos::prelude::*;

use crate::{
    components::{dialog::DialogComponent, page_loading::PageLoadingComponent},
    core::{
        api::{interests::get_all_interests, members::get_all_members_with_interests},
        models::interest::Interest,
    },
};

#[derive(Debug, Clone)]
struct LeptosSelectableInterest {
    interest: Interest,
    selected: RwSignal<bool>,
}

#[component]
pub fn MembersList() -> impl IntoView {
    let members = OnceResource::new(get_all_members_with_interests());

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

    let all_interests = OnceResource::new(get_all_interests());
    let selectable_interests = RwSignal::new(Vec::new());
    let interest_filter_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    Effect::new(move |_| {
        if let Some(Ok(interests)) = all_interests.get() {
            let mut result = vec![];
            for interest in interests {
                result.push(LeptosSelectableInterest {
                    interest,
                    selected: RwSignal::new(false),
                })
            }
            selectable_interests.set(result);
        }
    });

    let filtered_members = Memo::new(move |_| {
        if let Some(Ok(members_list)) = members.get() {
            let query = search_query.get().to_lowercase();
            let selected_interests = selectable_interests.with(|interests| {
                interests
                    .iter()
                    .filter(|s| s.selected.get())
                    .map(|s| s.interest.clone())
                    .collect::<Vec<_>>()
            });

            if query.is_empty() && selected_interests.is_empty() {
                Some(members_list)
            } else {
                Some(
                    members_list
                        .into_iter()
                        .filter(|(member, interests)| {
                            let full_name = format!(
                                "{} {} {}",
                                member.name, member.surname, member.second_surname
                            )
                            .to_lowercase();

                            let query_ok = if query.is_empty() {
                                true
                            } else {
                                full_name.contains(&query)
                                    || member.email.to_lowercase().contains(&query)
                                    || member.country.to_string().to_lowercase().contains(&query)
                                    || member.phone.to_string().to_lowercase().contains(&query)
                            };

                            let interest_ok = if selected_interests.is_empty() {
                                true
                            } else {
                                selected_interests
                                    .iter()
                                    .all(|selected| interests.contains(selected))
                            };

                            query_ok && interest_ok
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
                        <div class="flex justify-between gap-2">
                            <h2 class="card-title">"Todos los Socios"</h2>
                            <label class="input">
                                <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                    <g
                                    stroke-linejoin="round"
                                    stroke-linecap="round"
                                    stroke-width="2.5"
                                    fill="none"
                                    stroke="currentColor"
                                    >
                                    <circle cx="11" cy="11" r="8"></circle>
                                    <path d="m21 21-4.3-4.3"></path>
                                    </g>
                                </svg>
                                <input type="search" class="grow" placeholder="Buscar"
                                    on:input:target=move |ev| {
                                        search_bar.set(ev.target().value());
                                    }
                                    prop:value=search_bar/>
                            </label>
                            <div class="flex gap-2">
                                <a class="btn btn-primary" href="/members/new">"Añadir Socio"</a>
                                <button
                                class="btn btn-accent"
                                on:click=move |_| {
                                    let _ = interest_filter_dialog_ref_node.get().unwrap().show_modal();
                                }
                            >"Filtrar"</button>
                            </div>
                        </div>

                        <DialogComponent dialog_title="Filtrar" dialog_node_ref=interest_filter_dialog_ref_node dialog_content=move || {
                            view! {
                                <Show
                                    when=move || { !selectable_interests.get().is_empty() }
                                    fallback=|| view! { <p class="text-center">"No hay intereses..."</p> }
                                >
                                    <div class="flex flex-wrap gap-2">
                                        <For each=move || selectable_interests.get() key=|i| i.interest.id children=move |i| {
                                            view!{
                                                <label class="label">
                                                    <input type="checkbox" bind:checked=i.selected class="checkbox checkbox-primary" />
                                                    {i.interest.name}
                                                </label>
                                            }
                                        }/>
                                    </div>
                                </Show>
                            }
                        }/>

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
                                                        <For each=move || filtered_members.get().unwrap_or_default() key=|(m, _)| m.id children=move |(m, _)| {
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
