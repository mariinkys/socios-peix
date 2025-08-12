use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::{cupons::get_member_cupons, email::SendCuponEmail},
        utils::leptos::LeptosMemberCupon,
    },
};

#[component]
pub fn MemberCupons(edit_mode: RwSignal<bool>, member_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let model = RwSignal::new(LeptosMemberCupon::default());

    let member_cupons = Resource::new(
        move || member_id,
        |member_id| async move { get_member_cupons(member_id).await },
    );

    let send_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let send_email_action = ServerAction::<SendCuponEmail>::new();
    let send_value = send_email_action.value();
    Effect::new(move |_| {
        if let Some(val) = send_value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Enviado"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    model.set(LeptosMemberCupon::default());
                    member_cupons.refetch();
                    send_dialog_ref_node.get().unwrap().close();
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
    let items_per_page = RwSignal::new(5usize);
    let paginated_cupons = move || {
        if let Some(Ok(cupons)) = member_cupons.get() {
            let page = current_page.get();
            let per_page = items_per_page.get();
            let start_idx = (page - 1) * per_page;

            Some(
                cupons
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
        if let Some(Ok(cupons)) = member_cupons.get() {
            let total_items = cupons.len();
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
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Cupones"</h2>
                            <button
                                class="btn btn-accent"
                                disabled={move || !edit_mode.get() }
                                on:click=move |_| {
                                    let _ = send_dialog_ref_node.get().unwrap().show_modal();
                                }
                            >"Enviar"</button>
                            <DialogComponent dialog_title="Enviar Correo con Cupón" dialog_node_ref=send_dialog_ref_node dialog_content=move || {
                                view! {
                                    <div class="flex flex-col gap-2 w-full">
                                        <div class="w-full">
                                            <fieldset class="fieldset">
                                                <label class="label" for="subject">"Asunto"</label>
                                                <input type="text"
                                                    class="input w-full"
                                                    name="subject"
                                                    id="subject"
                                                    autocomplete="off"
                                                    disabled={move || !edit_mode.get() }
                                                    prop:value={move || model.get().email.subject}
                                                    on:input=move |ev| {
                                                        model.update(|curr| {
                                                            curr.email.subject = event_target_value(&ev);
                                                        });
                                                    }
                                                />
                                            </fieldset>
                                        </div>

                                        <div class="w-full">
                                            <fieldset class="fieldset">
                                                <label class="label" for="original_body">"Contenido"</label>
                                                <textarea
                                                    class="textarea w-full min-h-40"
                                                    name="original_body"
                                                    id="original_body"
                                                    autocomplete="off"
                                                    disabled={move || !edit_mode.get() }
                                                    prop:value={move || model.get().email.body}
                                                    on:input=move |ev| {
                                                        model.update(|curr| {
                                                            curr.email.body = event_target_value(&ev);
                                                        });
                                                    }
                                                />
                                            </fieldset>
                                        </div>

                                        <div class="w-full">
                                            <fieldset class="fieldset">
                                                <label class="label" for="cupon_description">"Descripción Cupón"</label>
                                                <textarea
                                                    class="textarea w-full min-h-40"
                                                    name="cupon_description"
                                                    id="cupon_description"
                                                    autocomplete="off"
                                                    disabled={move || !edit_mode.get() }
                                                    prop:value={move || model.get().cupon.description}
                                                    on:input=move |ev| {
                                                        model.update(|curr| {
                                                            curr.cupon.description = event_target_value(&ev);
                                                        });
                                                    }
                                                />
                                            </fieldset>
                                        </div>

                                        <div class="w-full">
                                            <fieldset class="fieldset">
                                                <label class="label" for="expiry_date">"Fecha de Caducidad"</label>
                                                <input type="date"
                                                    class="input w-full"
                                                    name="expiry_date"
                                                    id="expiry_date"
                                                    autocomplete="off"
                                                    disabled={move || !edit_mode.get() }
                                                    prop:value=move || {
                                                        model.get().cupon.expires_at
                                                            .map(|date| date.format("%Y-%m-%d").to_string())
                                                            .unwrap_or_default()
                                                        }
                                                    on:input=move |ev| {
                                                        let value = event_target_value(&ev);
                                                        model.update(|curr| {
                                                            if value.is_empty() {
                                                                curr.cupon.expires_at = None;
                                                            } else {
                                                                curr.cupon.expires_at = chrono::NaiveDate::parse_from_str(&value, "%Y-%m-%d").ok();
                                                            }
                                                        });
                                                    }
                                                />
                                            </fieldset>
                                        </div>

                                        <button disabled={ move || !edit_mode.get() } class="btn btn-primary"
                                            on:click={ move |_|{
                                                let model = model.get();
                                                send_email_action.dispatch(SendCuponEmail { member_id, cupon_description: model.cupon.description, cupon_expires_at: model.cupon.expires_at, subject: model.email.subject, original_body: model.email.body } );
                                            }}
                                        >"Enviar"</button>
                                    </div>
                                }
                            }/>
                        </div>
                        <div class="flex flex-col gap-2">
                            <Show
                                when=move || { paginated_cupons().is_some_and(|x| !x.is_empty()) }
                                fallback=|| view! { <p class="text-center">"No hay cupones..."</p> }
                            >
                                <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200">
                                    <table class="table">
                                        <thead>
                                        <tr>
                                            <th></th>
                                            <th>"Cupón"</th>
                                            <th>"Descripción"</th>
                                            <th>"Enviado el Día"</th>
                                            <th>"Caduca el Día"</th>
                                            <th>"Usado"</th>
                                            <th>"Cambiar Usado"</th>
                                        </tr>
                                        </thead>
                                        <tbody>
                                            <For each=move || paginated_cupons().unwrap_or_default() key=|c| c.id children=move |c| {
                                                view! {
                                                    <tr>
                                                        <th>{c.id}</th>
                                                        <td>{c.code.clone()}</td>
                                                        <td>{c.description}</td>
                                                        <td>{c.created_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                        <td>{c.expires_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                        <td>{if c.used { "Sí" } else { "No" }}</td>
                                                        <td> <a class="btn btn-accent w-min" href=format!("/cupon-check?cupon={}", c.code)>"Cambiar Usado"</a></td>
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
                        </div>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
