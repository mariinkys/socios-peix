use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{api::email::SendAllEmail, utils::leptos::LeptosEmail},
};

#[component]
pub fn AllEmails() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let email_model = RwSignal::new(LeptosEmail::default());

    let send_emails_action = ServerAction::<SendAllEmail>::new();
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
                    email_model.set(LeptosEmail::default());
                    dialog_ref_node.get().unwrap().close();
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
                <p class="text-xl text-center">"Escribir Correo a Todos"</p>
                <button class="btn btn-success w-full"
                    on:click=move |_| {
                        let _ = dialog_ref_node.get().unwrap().show_modal();
                    }
                >
                    "Escribir"
                </button>
            </div>
        </div>
        <DialogComponent dialog_title="Enviar Correo" dialog_node_ref=dialog_ref_node dialog_content=move || {
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
                                        prop:value={move || email_model.get().subject}
                                        on:input=move |ev| {
                                            email_model.update(|curr| {
                                                curr.subject = event_target_value(&ev);
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
                                        prop:value={move || email_model.get().body}
                                        on:input=move |ev| {
                                            email_model.update(|curr| {
                                                curr.body = event_target_value(&ev);
                                            });
                                        }
                                    />
                                </fieldset>
                            </div>
                        </div>
                        <div class="mt-3 flex flex-col gap-2 w-full">
                            <button class="btn btn-success w-full"
                                on:click={ move |_|{
                                    let email = email_model.get();
                                    send_emails_action.dispatch(SendAllEmail { subject: email.subject, original_body: email.body });
                                }}
                            >"Enviar"</button>
                        </div>
                    </ErrorBoundary>
                </Suspense>
            }
        }/>
    }
}
