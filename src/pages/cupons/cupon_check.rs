use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

use crate::{
    components::{
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::cupons::{CuponsByCode, SwapCuponUsed},
        models::cupon::CuponWithMember,
    },
};

#[derive(Params, PartialEq)]
struct CuponCheckParams {
    cupon: Option<String>,
}

#[component]
pub fn CuponCheckPage() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let cupons_model = RwSignal::new(Vec::<CuponWithMember>::new());
    let query = use_query::<CuponCheckParams>();

    let cupon_input = RwSignal::new(String::new());
    let search_input = RwSignal::new(String::new());
    Effect::new(move |_| {
        cupon_input.set(
            query
                .read()
                .as_ref()
                .ok()
                .and_then(|params| params.cupon.clone())
                .unwrap_or_default(),
        );
    });

    let search_by_code = ServerAction::<CuponsByCode>::new();
    let search_value = search_by_code.value();
    Effect::new(move |_| {
        if let Some(val) = search_value.get() {
            match val {
                Ok(res) => {
                    if res.is_empty() {
                        set_toast.set(ToastMessage {
                            message: String::from("No existe o esta caducado!"),
                            toast_type: ToastType::Error,
                            visible: true,
                        });
                    } else {
                        cupons_model.set(res);
                    }
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("Error: {err}"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        }
    });

    let filtered_cupons = move || {
        let cupons = cupons_model.get();
        if !cupons.is_empty() {
            let query = search_input.get().to_lowercase();

            if query.is_empty() {
                Some(cupons)
            } else {
                Some(
                    cupons
                        .into_iter()
                        .filter(|cupons| {
                            let full_name = cupons.member.get_full_name().to_lowercase();

                            if query.is_empty() {
                                true
                            } else {
                                full_name.contains(&query)
                                    || cupons.member.email.to_lowercase().contains(&query)
                                    || cupons
                                        .member
                                        .country
                                        .to_string()
                                        .to_lowercase()
                                        .contains(&query)
                                    || cupons
                                        .member
                                        .phone
                                        .to_string()
                                        .to_lowercase()
                                        .contains(&query)
                            }
                        })
                        .collect(),
                )
            }
        } else {
            None
        }
    };

    let swap_used_action = ServerAction::<SwapCuponUsed>::new();
    let swap_used_value = swap_used_action.value();
    Effect::new(move |_| {
        if let Some(val) = swap_used_value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Cambiado"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    cupons_model.set(Vec::<CuponWithMember>::new());
                    search_by_code.dispatch(CuponsByCode {
                        cupon: cupon_input.get_untracked(),
                    });
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("Error: {err}"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        }
    });

    let current_page = RwSignal::new(1usize);
    let items_per_page = RwSignal::new(20usize);
    // Reset to page 1 when filters change
    Effect::new(move |_| {
        search_input.track();
        current_page.set(1);
    });
    let paginated_cupons = move || {
        if let Some(cupons_list) = filtered_cupons() {
            let page = current_page.get();
            let per_page = items_per_page.get();
            let start_idx = (page - 1) * per_page;

            Some(
                cupons_list
                    .into_iter()
                    .skip(start_idx)
                    .take(per_page)
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    };
    let pagination_info = move || {
        if let Some(cupons_list) = filtered_cupons() {
            let total_items = cupons_list.len();
            let per_page = items_per_page.get();
            let total_pages = if total_items == 0 {
                1
            } else {
                total_items.div_ceil(per_page)
            };
            let current = current_page.get();

            (total_items, total_pages, current, per_page)
        } else {
            (0, 1, 1, items_per_page.get())
        }
    };

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="flex flex-row gap-2 mb-3 items-center">
                    <h2 class="text-2xl grow">"Comprobar Cupón"</h2>
                </div>

                <div class="flex justify-center w-full items-center">
                    <div class="flex flex-col md:flex-row gap-2 w-full md:w-1/2 items-center">
                        <fieldset class="fieldset w-full flex-1">
                            <input type="text"
                                class="input w-full"
                                name="subject"
                                id="subject"
                                placeholder="Cupón"
                                autocomplete="off"
                                prop:value={move || cupon_input.get()}
                                on:input=move |ev| {
                                    cupon_input.set(event_target_value(&ev));
                                }
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        let cupon = cupon_input.get();
                                        if !cupon.is_empty() {
                                            search_by_code.dispatch(CuponsByCode { cupon });
                                        } else {
                                            cupons_model.set(Vec::<CuponWithMember>::new());
                                        }
                                    }
                                }/>
                        </fieldset>

                        <div class="flex md:items-end w-full md:w-auto">
                            <button class="btn btn-primary w-full md:w-auto"
                                on:click={ move |_| {
                                    let cupon = cupon_input.get();
                                    if !cupon.is_empty() {
                                        search_by_code.dispatch(CuponsByCode { cupon });
                                    } else {
                                        cupons_model.set(Vec::<CuponWithMember>::new());
                                    }
                                }}
                            >
                                "Buscar"
                            </button>
                        </div>
                    </div>
                </div>

                <Show
                    when=move || { !cupons_model.get().is_empty() }
                    fallback=|| view! { <p>""</p> }
                >
                    <div class="card card-border bg-base-200 w-full mt-3">
                        <div class="card-body">
                            <label class="input w-full md:w-auto flex items-center">
                                <svg class="h-[1em] opacity-50 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                    <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
                                        <circle cx="11" cy="11" r="8"></circle>
                                        <path d="m21 21-4.3-4.3"></path>
                                    </g>
                                </svg>
                                <input type="search" class="grow" placeholder="Buscar"
                                    on:input:target=move |ev| {
                                        search_input.set(ev.target().value());
                                    }
                                    prop:value=search_input />
                            </label>
                        </div>
                    </div>
                </Show>

                <Show
                    when=move || { paginated_cupons().is_some_and(|x| !x.is_empty())}
                    fallback=|| view! { <p class="text-center mt-3">"No hay cupones..."</p> }
                >
                    <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200 mt-3">
                        <table class="table">
                            <thead>
                            <tr>
                                <th></th>
                                <th>"Cupón"</th>
                                <th>"Pertenece"</th>
                                <th>"Descripción"</th>
                                <th>"Enviado el Día"</th>
                                <th>"Caduca el Día"</th>
                                <th>"Usado"</th>
                                <th>"Cambiar Usado"</th>
                            </tr>
                            </thead>
                            <tbody>
                                <For each=move || paginated_cupons().unwrap_or_default() key=|c| c.cupon.id children=move |c| {
                                    view! {
                                        <tr>
                                            <th>{c.cupon.id}</th>
                                            <td>{c.cupon.code}</td>
                                            <td>{c.member.get_full_name()}</td>
                                            <td>{c.cupon.description}</td>
                                            <td>{c.cupon.created_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                            <td>{c.cupon.expires_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                            <td>{if c.cupon.used { "Sí" } else { "No" }}</td>
                                            <td><button class="btn btn-accent"
                                                    on:click={ move |_| {
                                                        if let Some(id) = c.cupon.id {
                                                            swap_used_action.dispatch(SwapCuponUsed { cupon_id: id, current_value: c.cupon.used });
                                                        }
                                                    }}
                                                >
                                                    "Cambiar Usado"
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                }/>
                            </tbody>
                        </table>
                    </div>

                    <div class="flex justify-center mt-4">
                        <div class="join">
                            {move || {
                                let (_, total_pages, current, _) = pagination_info();

                                let prev_disabled = current <= 1;
                                let prev_button = view! {
                                    <button class="join-item btn"
                                        class:btn-disabled=prev_disabled
                                        on:click=move |_| {
                                            if current > 1 {
                                                current_page.set(current - 1);
                                            }
                                        }
                                    >"«"</button>
                                };

                                // Page numbers
                                let mut page_buttons = Vec::new();
                                let start_page = if current <= 3 { 1 } else { current - 2 };
                                let end_page = std::cmp::min(start_page + 4, total_pages);
                                let actual_start = if end_page - start_page < 4 && end_page >= 5 {
                                    std::cmp::max(1, end_page - 4)
                                } else {
                                    start_page
                                };

                                for page in actual_start..=end_page {
                                    let is_current = page == current;
                                    page_buttons.push(view! {
                                        <button class="join-item btn"
                                            class:btn-active=is_current
                                            on:click=move |_| {
                                                current_page.set(page);
                                            }
                                        >{page}</button>
                                    });
                                }

                                let next_disabled = current >= total_pages;
                                let next_button = view! {
                                    <button class="join-item btn"
                                        class:btn-disabled=next_disabled
                                        on:click=move |_| {
                                            if current < total_pages {
                                                current_page.set(current + 1);
                                            }
                                        }
                                    >"»"</button>
                                };

                                view! {
                                    <div class="flex gap-1">
                                        {prev_button}
                                        <div>
                                            {page_buttons.into_iter().collect_view()}
                                        </div>
                                        {next_button}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </Show>
            </ErrorBoundary>
        </Suspense>
    }
}
