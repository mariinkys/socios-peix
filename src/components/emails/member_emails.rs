use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::email::{SendSingleEmail, get_member_emails},
        utils::leptos::LeptosEmail,
    },
};

#[component]
pub fn MemberEmails(edit_mode: RwSignal<bool>, member_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let model = RwSignal::new(LeptosEmail::default());

    let member_emails = Resource::new(
        move || member_id,
        |member_id| async move { get_member_emails(member_id).await },
    );

    let send_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let send_email_action = ServerAction::<SendSingleEmail>::new();
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
                    model.set(LeptosEmail::default());
                    member_emails.refetch();
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
    let paginated_emails = move || {
        if let Some(Ok(cupons)) = member_emails.get() {
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
        if let Some(Ok(cupons)) = member_emails.get() {
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
                            <h2 class="card-title grow">"Emails"</h2>
                            <button
                                class="btn btn-accent"
                                disabled={move || !edit_mode.get() }
                                on:click=move |_| {
                                    let _ = send_dialog_ref_node.get().unwrap().show_modal();
                                }
                            >"Enviar"</button>
                            <DialogComponent dialog_title="Enviar Correo" dialog_node_ref=send_dialog_ref_node dialog_content=move || {
                                view! {
                                    <ActionForm action=send_email_action>
                                        <div class="flex flex-col gap-2 w-full">
                                            // We need the id for the update but we don't want to show it.
                                            <input type="hidden" name="member_id" autocomplete="off"  prop:value={member_id}/>
                                            <div class="w-full">
                                                <fieldset class="fieldset">
                                                    <label class="label" for="subject">"Asunto"</label>
                                                    <input type="text"
                                                        class="input w-full"
                                                        name="subject"
                                                        id="subject"
                                                        autocomplete="off"
                                                        disabled={move || !edit_mode.get() }
                                                        prop:value={move || model.get().subject}
                                                        on:input=move |ev| {
                                                            model.update(|curr| {
                                                                curr.subject = event_target_value(&ev);
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
                                                        prop:value={move || model.get().body}
                                                        on:input=move |ev| {
                                                            model.update(|curr| {
                                                                curr.body = event_target_value(&ev);
                                                            });
                                                        }
                                                    />
                                                </fieldset>
                                            </div>

                                            <button disabled={ move || !edit_mode.get() } type="submit" class="btn btn-primary">"Enviar"</button>
                                        </div>
                                    </ActionForm>
                                }
                            }/>
                        </div>
                        <div class="flex flex-col gap-2">
                            <Show
                                when=move || { paginated_emails().is_some_and(|x| !x.is_empty()) }
                                fallback=|| view! { <p class="text-center">"No hay emails..."</p> }
                            >
                                <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200">
                                    <table class="table">
                                        <thead>
                                        <tr>
                                            <th></th>
                                            <th>"Asunto"</th>
                                            <th>"Contenido"</th>
                                            <th>"Enviado el Día"</th>
                                        </tr>
                                        </thead>
                                        <tbody>
                                            <For each=move || paginated_emails().unwrap_or_default() key=|e| e.id children=move |e| {
                                                view! {
                                                    <tr>
                                                        <th>{e.id}</th>
                                                        <td>{e.subject}</td>
                                                        <td>{e.body}</td>
                                                        <td>{e.created_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
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
