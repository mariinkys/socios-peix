use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{api::email::SendAllCuponEmail, utils::leptos::LeptosMemberCupon},
};

#[component]
pub fn AllCuponEmails() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let model = RwSignal::new(LeptosMemberCupon::default());

    let send_emails_action = ServerAction::<SendAllCuponEmail>::new();
    let send_value = send_emails_action.value();
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

    view! {
        <div class="card bg-base-100 shadow-xl w-80">
            <div class="card-body w-full h-full justify-between">
                <p class="text-xl text-center">"Enviar Cupón a Todos"</p>
                <button class="btn btn-success w-full"
                    on:click=move |_| {
                        let _ = dialog_ref_node.get().unwrap().show_modal();
                    }
                >
                    "Enviar"
                </button>
            </div>
        </div>
        <DialogComponent dialog_title="Enviar Cupón" dialog_node_ref=dialog_ref_node dialog_content=move || {
            view! {
                <Suspense fallback=move || view! { <PageLoadingComponent/> }>
                    <ErrorBoundary fallback=|error| view! {
                        <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
                    }>
                        <div class="flex flex-col gap-2 w-full">
                            <div class="w-full">
                                <fieldset class="fieldset">
                                    <label class="label" for="subject">"Asunto"</label>
                                    <input type="text"
                                        class="input w-full"
                                        name="subject"
                                        id="subject"
                                        autocomplete="off"
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
                                    <label class="label" for="body">"Contenido"</label>
                                    <textarea
                                        class="textarea w-full min-h-40"
                                        name="body"
                                        id="body"
                                        autocomplete="off"
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
                        </div>
                        <div class="mt-3 flex flex-col gap-2 w-full">
                            <button class="btn btn-success w-full"
                                on:click={ move |_|{
                                    let model = model.get();
                                    send_emails_action.dispatch(SendAllCuponEmail { cupon_description: model.cupon.description, cupon_expires_at: model.cupon.expires_at, subject: model.email.subject, original_body: model.email.body });
                                }}
                            >"Enviar"</button>
                        </div>
                    </ErrorBoundary>
                </Suspense>
            }
        }/>
    }
}
