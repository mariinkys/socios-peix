use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::{interests::get_all_interests, members::get_all_members_with_interests},
        models::{interest::Interest, member::MemberWithInterests},
        utils::generate_members_excel,
    },
};

#[derive(Debug, Clone)]
struct LeptosSelectableInterest {
    interest: Interest,
    selected: RwSignal<bool>,
}

#[component]
pub fn MembersList() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
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

    let filtered_members = move || {
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
                        .filter(|member_with_interests| {
                            let full_name = format!(
                                "{} {} {}",
                                member_with_interests.member.name,
                                member_with_interests.member.surname,
                                member_with_interests.member.second_surname
                            )
                            .to_lowercase();

                            let query_ok = if query.is_empty() {
                                true
                            } else {
                                full_name.contains(&query)
                                    || member_with_interests
                                        .member
                                        .email
                                        .to_lowercase()
                                        .contains(&query)
                                    || member_with_interests
                                        .member
                                        .country
                                        .to_string()
                                        .to_lowercase()
                                        .contains(&query)
                                    || member_with_interests
                                        .member
                                        .phone
                                        .to_string()
                                        .to_lowercase()
                                        .contains(&query)
                            };

                            let interest_ok = if selected_interests.is_empty() {
                                true
                            } else {
                                selected_interests.iter().all(|selected| {
                                    member_with_interests.interests.contains(selected)
                                })
                            };

                            query_ok && interest_ok
                        })
                        .collect(),
                )
            }
        } else {
            None
        }
    };

    let export_excel_action = Action::new(move |model: &Option<Vec<MemberWithInterests>>| {
        let current_model = model.clone();
        async move {
            if let Some(model) = current_model {
                let result = generate_members_excel(model).await;

                match result {
                    Ok(base64_excel) => {
                        let download_excel = move || {
                            let window = leptos::prelude::window();
                            let document = window.document().unwrap();

                            let binary_string = window.atob(&base64_excel).unwrap();
                            let bytes = leptos::web_sys::js_sys::Uint8Array::new_with_length(
                                binary_string.len() as u32,
                            );

                            for (i, char) in binary_string.chars().enumerate() {
                                bytes.set_index(i as u32, char as u8);
                            }

                            let array = leptos::web_sys::js_sys::Array::new();
                            array.push(&bytes);
                            let blob =
                                leptos::web_sys::Blob::new_with_u8_array_sequence(&array).unwrap();
                            let url =
                                leptos::web_sys::Url::create_object_url_with_blob(&blob).unwrap();

                            let a = document.create_element("a").unwrap();
                            a.set_attribute("href", &url).unwrap();
                            a.set_attribute("download", "export.xlsx").unwrap();

                            if let Some(a_element) =
                                wasm_bindgen::JsCast::dyn_ref::<leptos::web_sys::HtmlElement>(&a)
                            {
                                document.body().unwrap().append_child(&a).unwrap();
                                a_element.click();
                                document.body().unwrap().remove_child(&a).unwrap();
                                leptos::web_sys::Url::revoke_object_url(&url).unwrap();
                            } else {
                                leptos::web_sys::Url::revoke_object_url(&url).unwrap();
                            }
                        };

                        download_excel();
                    }
                    Err(err) => {
                        set_toast.set(ToastMessage {
                            message: format!("Error Exporting {err}"),
                            toast_type: ToastType::Error,
                            visible: true,
                        });
                    }
                }
            } else {
                set_toast.set(ToastMessage {
                    message: String::from("No data to export"),
                    toast_type: ToastType::Error,
                    visible: true,
                });
            }
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
                                <button
                                    class="btn btn-success"
                                    on:click=move |_| {
                                        export_excel_action.dispatch(filtered_members());
                                    }
                                >"Exportar"</button>
                                <a class="btn btn-primary" href="/members/new">"Añadir"</a>
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

                        <Show
                            when=move || { filtered_members().is_some_and(|x| !x.is_empty())}
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
                                        <For each=move || filtered_members().unwrap_or_default() key=|m| m.member_id children=move |m| {
                                            view! {
                                                <tr>
                                                    <th>{m.member_id.unwrap_or_default()}</th>
                                                    <td>{format!("{} {} {}", m.member.name, m.member.surname, m.member.second_surname)}</td>
                                                    <td>{m.member.email}</td>
                                                    <td>{m.member.birthdate.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                    <td>{m.member.gender.to_string()}</td>
                                                    <td>{m.member.country.to_string()}</td>
                                                    <td class="text-right"><a class="btn btn-primary" href=format!("/members/{}", m.member_id.unwrap_or_default())>"Ver"</a></td>
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
